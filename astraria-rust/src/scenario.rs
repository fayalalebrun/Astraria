use crate::{AstrariaError, AstrariaResult};
use glam::DVec3;

#[derive(Debug, Clone, PartialEq)]
pub enum BodyType {
    Planet {
        radius: f32,
        texture_path: String,
    },
    Star {
        radius: f32,
        texture_path: String,
        temperature: f32,
    },
    PlanetAtmo {
        radius: f32,
        texture_path: String,
        atmo_color: [f32; 4],
        ambient_texture: Option<String>,
    },
    BlackHole {
        radius: f32,
    },
}

#[derive(Debug, Clone)]
pub struct ScenarioBody {
    pub name: String,
    pub mass: f64,
    pub position: DVec3,
    pub velocity: DVec3,
    pub body_type: BodyType,
    pub orbit_color: [f32; 4],
    pub rotation_params: (f32, f32, f32, f32), // incTilt, axisRightAsc, rotPeriod, offset (all in radians)
}

#[derive(Debug)]
pub struct Scenario {
    pub bodies: Vec<ScenarioBody>,
}

pub struct ScenarioParser;

impl ScenarioParser {
    pub fn parse(content: &str) -> AstrariaResult<Scenario> {
        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() || !lines[0].trim().eq("v3") {
            return Err(AstrariaError::ParseError(
                "Invalid scenario file format. Expected 'v3' header.".to_string(),
            ));
        }

        let mut bodies = Vec::new();
        let mut i = 1;

        while i < lines.len() {
            let line = lines[i].trim();

            // Skip empty lines
            if line.is_empty() {
                i += 1;
                continue;
            }

            // Parse object type
            if line.starts_with("type:") {
                let object_type = Self::extract_value(line)?;

                match object_type.as_str() {
                    "planet" => {
                        if let Ok(body) = Self::parse_planet(&lines, &mut i) {
                            bodies.push(body);
                        }
                    }
                    "star" => {
                        if let Ok(body) = Self::parse_star(&lines, &mut i) {
                            bodies.push(body);
                        }
                    }
                    "planet_atmo" => {
                        if let Ok(body) = Self::parse_planet_atmo(&lines, &mut i) {
                            bodies.push(body);
                        }
                    }
                    "black_hole" => {
                        if let Ok(body) = Self::parse_black_hole(&lines, &mut i) {
                            bodies.push(body);
                        }
                    }
                    _ => {
                        log::warn!("Unknown object type: {}", object_type);
                        i += 1;
                    }
                }
            } else {
                i += 1;
            }
        }

        Ok(Scenario { bodies })
    }

    fn parse_planet(lines: &[&str], i: &mut usize) -> AstrariaResult<ScenarioBody> {
        *i += 1; // Move past type line

        let name = Self::extract_value(lines[*i])?;
        *i += 1;

        let radius = Self::extract_value(lines[*i])?.parse::<f32>()?;
        *i += 1;

        let mass = Self::extract_value(lines[*i])?.parse::<f64>()?;
        *i += 1;

        let velocity = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let position = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let texture_path = Self::extract_value(lines[*i])?;
        *i += 1;

        let orbit_color = Self::parse_color4(lines[*i])?;
        *i += 1;

        let rotation_params = Self::parse_rotation(lines[*i])?;
        *i += 1;

        Ok(ScenarioBody {
            name,
            mass,
            position,
            velocity,
            body_type: BodyType::Planet {
                radius,
                texture_path,
            },
            orbit_color,
            rotation_params,
        })
    }

    fn parse_star(lines: &[&str], i: &mut usize) -> AstrariaResult<ScenarioBody> {
        *i += 1; // Move past type line

        let name = Self::extract_value(lines[*i])?;
        *i += 1;

        let radius = Self::extract_value(lines[*i])?.parse::<f32>()?;
        *i += 1;

        let mass = Self::extract_value(lines[*i])?.parse::<f64>()?;
        *i += 1;

        let velocity = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let position = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let texture_path = Self::extract_value(lines[*i])?;
        *i += 1;

        let orbit_color = Self::parse_color4(lines[*i])?;
        *i += 1;

        let rotation_params = Self::parse_rotation(lines[*i])?;
        *i += 1;

        let temperature = Self::extract_value(lines[*i])?.parse::<f32>()?;
        *i += 1;

        Ok(ScenarioBody {
            name,
            mass,
            position,
            velocity,
            body_type: BodyType::Star {
                radius,
                texture_path,
                temperature,
            },
            orbit_color,
            rotation_params,
        })
    }

    fn parse_planet_atmo(lines: &[&str], i: &mut usize) -> AstrariaResult<ScenarioBody> {
        *i += 1; // Move past type line

        let name = Self::extract_value(lines[*i])?;
        *i += 1;

        let radius = Self::extract_value(lines[*i])?.parse::<f32>()?;
        *i += 1;

        let mass = Self::extract_value(lines[*i])?.parse::<f64>()?;
        *i += 1;

        let velocity = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let position = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let texture_path = Self::extract_value(lines[*i])?;
        *i += 1;

        let orbit_color = Self::parse_color4(lines[*i])?;
        *i += 1;

        let rotation_params = Self::parse_rotation(lines[*i])?;
        *i += 1;

        let atmo_color = Self::parse_color4(lines[*i])?;
        *i += 1;

        // Check for optional ambient texture
        let mut ambient_texture = None;
        if *i < lines.len() && lines[*i].starts_with("ambientTexture:") {
            ambient_texture = Some(Self::extract_value(lines[*i])?);
            *i += 1;
        }

        Ok(ScenarioBody {
            name,
            mass,
            position,
            velocity,
            body_type: BodyType::PlanetAtmo {
                radius,
                texture_path,
                atmo_color,
                ambient_texture,
            },
            orbit_color,
            rotation_params,
        })
    }

    fn parse_black_hole(lines: &[&str], i: &mut usize) -> AstrariaResult<ScenarioBody> {
        *i += 1; // Move past type line

        let name = Self::extract_value(lines[*i])?;
        *i += 1;

        let radius = Self::extract_value(lines[*i])?.parse::<f32>()?;
        *i += 1;

        let mass = Self::extract_value(lines[*i])?.parse::<f64>()?;
        *i += 1;

        let velocity = Self::parse_vec3(lines[*i])?;
        *i += 1;

        let position = Self::parse_vec3(lines[*i])?;
        *i += 1;

        // Black holes don't have textures or colors in the basic format
        let orbit_color = [0.0, 0.0, 0.0, 1.0]; // Default black
        let rotation_params = (0.0, 0.0, 0.0, 0.0); // No rotation

        Ok(ScenarioBody {
            name,
            mass,
            position,
            velocity,
            body_type: BodyType::BlackHole { radius },
            orbit_color,
            rotation_params,
        })
    }

    fn extract_value(line: &str) -> AstrariaResult<String> {
        if let Some(colon_pos) = line.find(':') {
            Ok(line[colon_pos + 1..].trim().to_string())
        } else {
            Err(AstrariaError::ParseError(format!(
                "Invalid line format: {}",
                line
            )))
        }
    }

    fn parse_vec3(line: &str) -> AstrariaResult<DVec3> {
        let values = Self::extract_value(line)?;
        let parts: Vec<&str> = values.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(AstrariaError::ParseError(format!(
                "Expected 3 values for Vec3, got {}",
                parts.len()
            )));
        }

        Ok(DVec3::new(
            parts[0].parse::<f64>()?,
            parts[1].parse::<f64>()?,
            parts[2].parse::<f64>()?,
        ))
    }

    fn parse_color4(line: &str) -> AstrariaResult<[f32; 4]> {
        let values = Self::extract_value(line)?;
        let parts: Vec<&str> = values.split_whitespace().collect();

        if parts.len() != 4 {
            return Err(AstrariaError::ParseError(format!(
                "Expected 4 values for color, got {}",
                parts.len()
            )));
        }

        Ok([
            parts[0].parse::<f32>()?,
            parts[1].parse::<f32>()?,
            parts[2].parse::<f32>()?,
            parts[3].parse::<f32>()?,
        ])
    }

    fn parse_rotation(line: &str) -> AstrariaResult<(f32, f32, f32, f32)> {
        let values = Self::extract_value(line)?;
        let parts: Vec<&str> = values.split_whitespace().collect();

        if parts.len() != 4 {
            return Err(AstrariaError::ParseError(format!(
                "Expected 4 values for rotation, got {}",
                parts.len()
            )));
        }

        // Convert degrees to radians as per Java implementation
        Ok((
            parts[0].parse::<f32>()?.to_radians(),
            parts[1].parse::<f32>()?.to_radians(),
            parts[2].parse::<f32>()?.to_radians(),
            parts[3].parse::<f32>()?.to_radians(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_scenario() {
        let content = r#"v3

type: star
name: Sun
radius: 695700.0
mass: 1.9890984042E30
velocity: -8.095051963673479 10.079420705729289 0.1813033769140992
position: 3.857986024050543E8 7.962288116906488E8 -2.0698254349011008E7
texture: ./Planet Textures/2k_sun.jpg
orbit_color: 0.8901961 0.6509804 0.0 0.8
rotation: 7.25 331.15 14.18 0
temperature: 5778

type: planet
name: Earth
radius: 6378.1
mass: 5.9723E24
velocity: 21105.314172210237 20398.1878091857 2.087430514166044E-4
position: 1.0482728064552127E11 -1.0927061102891383E11 -1.6817723556683976E7
texture: ./Planet Textures/8k_earth_with_clouds.jpg
orbit_color: 0.18039216 0.43137255 0.6392157 0.8
rotation: 23.440000000000005 90.0 360.98562350000003 -10
"#;

        let scenario = ScenarioParser::parse(content).unwrap();
        assert_eq!(scenario.bodies.len(), 2);

        // Test star
        let sun = &scenario.bodies[0];
        assert_eq!(sun.name, "Sun");
        assert!(matches!(
            sun.body_type,
            BodyType::Star {
                temperature: 5778.0,
                ..
            }
        ));

        // Test planet
        let earth = &scenario.bodies[1];
        assert_eq!(earth.name, "Earth");
        assert!(matches!(earth.body_type, BodyType::Planet { .. }));
    }
}
