use std::collections::HashMap;
use cgmath::{Point3, Vector2};
use peg::{error::ParseError, str::LineCol};

use crate::data::{TexParams, BrushPlane, Brush, Entity, Map};

peg::parser! {
    grammar map_grammar() for str {
        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "trace")]
                println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        pub rule commentline() -> String
            = "//" " "* x:$([^'\n' | '\r']+) { x.to_owned() }

        pub rule linebreak() = "\n" / "\r\n"; // / ![_];
        rule whitespace() = [' ' | '\t'];
        pub rule linesep() = (linebreak() / commentline() / whitespace())+// (![_])?;

        pub rule keypair() -> (String, String)
            = "\"" k:$([^'"']+) "\"" " "+  "\"" v:$([^'"']+) "\"" { (k.to_owned(), v.to_owned()) }
        pub rule keys() -> HashMap<String, String>
            = kps:keypair() ++ linesep() { kps.into_iter().collect() }

        rule float32() -> f32
            = quiet!{ text:$("-"? ['0'..='9']+ ("." ['0'..='9']*)?) { text.parse().unwrap() } }
            / expected!("float")

        rule fpoint3() -> Point3<f32>
            = "( " x:float32() " " y:float32() " " z:float32() " )" { Point3::new(x, y, z) }

        pub rule brushline() -> BrushPlane
            = p:fpoint3() " " q:fpoint3() " " r:fpoint3()
              " " texname:$(['a'..='z' | 'A'..='Z' | '_']+)
              " " tex_ox:float32() " " tex_oy:float32()
              " " tex_rot:float32()
              " " tex_sx:float32() " " tex_sy:float32()
            {
                BrushPlane {
                    p, q, r,
                    texname: texname.to_owned(),
                    texparams: TexParams {
                        off: Vector2::new(tex_ox, tex_oy),
                        rot: tex_rot,
                        scale: Vector2::new(tex_sx, tex_sy),
                    },
                }
            }
        
        pub rule brush() -> Brush
            = "{" linesep()
              bls:brushline() ** linesep()
              linesep() "}"
            {
                Brush {
                    planes: bls,
                }
            }
        
        pub rule entity() -> Entity
            = "{" linesep()
              keys:keys() linesep()
              brushes:brush() ** linesep()
              linesep()? "}"
            {
                Entity { keys, brushes }
            };
        
        pub rule map() -> Map
            =
              linesep()*
              ents:entity() ++ linesep()
              linesep()*
            {
                Map { entities: ents }
            };
    }
}

pub fn parse(x: impl AsRef<str>) -> Result<Map, ParseError<LineCol>> {
    map_grammar::map(x.as_ref())
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use cgmath::assert_ulps_eq;

    /// Tests keypair parsing for entity k/v entries.
    #[test]
    fn keypair() {
        let x = map_grammar::keypair("\"k\" \"v\"");
        assert_eq!(x.unwrap(), ("k".to_owned(), "v".to_owned()));
    }

    /// Tests the line separation parsing
    #[test]
    fn linesep() {
        let x = map_grammar::linesep("\n    ");
        assert!(x.is_ok());
    }

    /// Tests parsing the keys blob of an entity
    #[test]
    fn keys() {
        let inp = r#""spawnflags" "0"
"classname" "worldspawn"
"wad" "E:\q1maps\Q.wad""#;

        let x = map_grammar::keys(inp).unwrap();
        assert_eq!(x["classname"], "worldspawn");
        assert_eq!(x["wad"], "E:\\q1maps\\Q.wad");
        assert_eq!(x["spawnflags"], "0");
    }

    /// Tests that we can parse a single line of brush description
    #[test]
    fn brushline() {
        let inp = "( -64 -64 -16 ) ( -64 -63 -16 ) ( -64 -64 -15 ) __TB_empty 0 0 0 1 1";
        let x = map_grammar::brushline(inp).unwrap();
        assert_ulps_eq!(x.p, Point3::new(-64., -64., -16.));
        assert_ulps_eq!(x.q, Point3::new(-64., -63., -16.));
        assert_ulps_eq!(x.r, Point3::new(-64., -64., -15.));
        assert_eq!(x.texname, "__TB_empty");
        assert_ulps_eq!(x.texparams.off, Vector2::new(0., 0.));
        assert_ulps_eq!(x.texparams.scale, Vector2::new(1., 1.));
        assert_ulps_eq!(x.texparams.rot, 0.);
    }

    #[test]
    fn brush() {
        let inp =
r#"{
( -16 0 -48 ) ( -16 1 -48 ) ( -16 0 -47 ) __TB_empty 0 0 0 1 1
( -16 0 -48 ) ( -16 0 -47 ) ( -15 0 -48 ) __TB_empty 0 0 0 1 1
( -16 0 -48 ) ( -15 0 -48 ) ( -16 1 -48 ) __TB_empty 0 0 0 1 1
( 80 80 -32 ) ( 80 81 -32 ) ( 81 80 -32 ) __TB_empty 0 0 0 1 1
( 80 96 -32 ) ( 81 96 -32 ) ( 80 96 -31 ) __TB_empty 0 0 0 1 1
( 80 80 -32 ) ( 80 80 -31 ) ( 80 81 -32 ) __TB_empty 0 0 0 1 1
}"#;
        let x = map_grammar::brush(inp).unwrap();
        assert_ulps_eq!(x.planes[0].q, Point3::new(-16., 1., -48.));
        assert_ulps_eq!(x.planes[3].r, Point3::new(81., 80., -32.));
        assert_ulps_eq!(x.planes[5].p, Point3::new(80., 80., -32.));
    }

    #[test]
    fn entity() {
        let inp =
r#"{
"classname" "worldspawn"
{
( -16 0 -48 ) ( -16 1 -48 ) ( -16 0 -47 ) __TB_empty 0 0 0 1 1
( -16 0 -48 ) ( -16 0 -47 ) ( -15 0 -48 ) __TB_empty 0 0 0 1 1
( -16 0 -48 ) ( -15 0 -48 ) ( -16 1 -48 ) __TB_empty 0 0 0 1 1
( 80 80 -32 ) ( 80 81 -32 ) ( 81 80 -32 ) __TB_empty 0 0 0 1 1
( 80 96 -32 ) ( 81 96 -32 ) ( 80 96 -31 ) __TB_empty 0 0 0 1 1
( 80 80 -32 ) ( 80 80 -31 ) ( 80 81 -32 ) __TB_empty 0 0 0 1 1
}
}"#;
        let x = map_grammar::entity(inp).unwrap();
        assert_eq!(x.keys["classname"], "worldspawn");
        let brush = &x.brushes[0];
        assert_ulps_eq!(brush.planes[0].q, Point3::new(-16., 1., -48.));
        assert_ulps_eq!(brush.planes[3].r, Point3::new(81., 80., -32.));
        assert_ulps_eq!(brush.planes[5].p, Point3::new(80., 80., -32.));

        let inp =
r#"{
"classname" "monster_dog"
"origin" "-16 -32 40"
"angle" "50"
}"#;
        let x = map_grammar::entity(inp).unwrap();
        assert_eq!(x.keys["classname"], "monster_dog");
        assert_eq!(x.keys["origin"], "-16 -32 40");
        assert_eq!(x.keys["angle"], "50");
    }

    #[test]
    fn fullmap() {
        let inp =
r#"// Game: Quake
// Format: Valve
// entity 0
{
"mapversion" "220"
"classname" "worldspawn"
// brush 0
{
( -64 -64 -16 ) ( -64 -63 -16 ) ( -64 -64 -15 ) __TB_empty 0 0 0 1 1
( -64 -64 -16 ) ( -64 -64 -15 ) ( -63 -64 -16 ) __TB_empty 0 0 0 1 1
( -64 -64 -16 ) ( -63 -64 -16 ) ( -64 -63 -16 ) __TB_empty 0 0 0 1 1
( 64 64 16 ) ( 64 65 16 ) ( 65 64 16 ) __TB_empty 0 0 0 1 1
( 64 64 16 ) ( 65 64 16 ) ( 64 64 17 ) __TB_empty 0 0 0 1 1
( 64 64 16 ) ( 64 64 17 ) ( 64 65 16 ) __TB_empty 0 0 0 1 1
}
// brush 1
{
( -16 0 -16 ) ( -16 1 -16 ) ( -16 0 -15 ) __TB_empty 0 0 0 1 1
( -16 0 -16 ) ( -16 0 -15 ) ( -15 0 -16 ) __TB_empty 0 0 0 1 1
( -16 0 -16 ) ( -15 0 -16 ) ( -16 1 -16 ) __TB_empty 0 0 0 1 1
( 80 80 0 ) ( 80 81 0 ) ( 81 80 0 ) __TB_empty 0 0 0 1 1
( 80 96 0 ) ( 81 96 0 ) ( 80 96 1 ) __TB_empty 0 0 0 1 1
( 80 80 0 ) ( 80 80 1 ) ( 80 81 0 ) __TB_empty 0 0 0 1 1
}
}
// entity 1
{
"classname" "monster_dog"
"origin" "-16 -32 40"
"angle" "50"
}
// entity 2
{
"classname" "weapon_supershotgun"
"origin" "48 -32 16"
"angle" "270"
}"#;
        let map = map_grammar::map(inp).unwrap();
        assert_eq!(map.entities[0].keys["mapversion"], "220");
        assert_ulps_eq!(map.entities[0].brushes[1].planes[0].p, Point3::new(-16., 0., -16.));
        assert_eq!(map.entities[2].keys["classname"], "weapon_supershotgun");
        assert_eq!(map.entities[2].keys["origin"], "48 -32 16");
    }
}