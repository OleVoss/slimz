use std::ops::Index;

use bevy::prelude::*;

use crate::{Particle, SensorControll, SensorPosition, TrailMap, WorldDimensions};

pub const PARTICLE_SPEED: i32 = 5;

pub fn movement(
    mut query: Query<(&mut Particle, &SensorControll)>,
    dimensions: Res<WorldDimensions>,
    trail_map: Res<TrailMap>,
) {
    for (mut particle, sensors) in query.iter_mut() {

        particle.heading = sense(sensors, &particle, &trail_map, &dimensions);
        let direction_vector = angle_to_unit_vec(particle.heading);

        let new_x = particle.x + direction_vector.x as i32 * PARTICLE_SPEED;
        let new_y = particle.y + direction_vector.y as i32 * PARTICLE_SPEED;
        
        if new_x.is_negative() {
            particle.x = new_x + dimensions.width;
        } else {
            particle.x = new_x % dimensions.width;
        }

        if new_y.is_negative() {
            particle.y = new_y + dimensions.height;
        } else {
            particle.y = new_y % dimensions.height;
        }
    }
}

fn sense(
    sensors: &SensorControll,
    particle: &Particle,
    trail_map: &TrailMap,
    dimensions: &WorldDimensions,
) -> f32 {
    let mut sensor_readings = Vec::<f32>::new();
    for sensor in &sensors.sensors {
        let mut sensor_vec = angle_to_unit_vec(sensor.angle + particle.heading);
        sensor_vec *= sensor.distance; 

        let sensor_x = (particle.x + sensor_vec[0] as i32) % dimensions.width;
        let sensor_y = (particle.y + sensor_vec[1] as i32) % dimensions.height;

        let index = sensor_y * dimensions.width as i32 + sensor_x;

        let sensor_reading = readings_3x3(&trail_map.trail, index as usize, dimensions);
        sensor_readings.push(sensor_reading);
    }
    let f = sensor_readings[1];
    let fl = sensor_readings[0];
    let fr = sensor_readings[2];

    let angle = 10.0;
    if f > fl && f > fr {
        particle.heading
    } else if f < fl && f < fr {
        if rand::random() {
            particle.heading + angle
        } else {
            particle.heading - angle
        }
    } else if fl < fr {
        particle.heading - angle
    } else if fl > fr {
        particle.heading + angle
    } else {
        particle.heading
    }

}

pub fn trail_map(
    query: Query<&Particle>,
    mut trail_map: ResMut<TrailMap>,
    dimensions: Res<WorldDimensions>,
) {
    let trail_buffer = trail_map.trail.clone();

    // diffuse
    trail_map
        .trail
        .iter_mut()
        .enumerate()
        .for_each(|(i, cell)| {
            let reading = readings_3x3(&trail_buffer, i, &dimensions) / 9.0;
            if reading > 0.5 { // evaporation threshold
                *cell = reading;
            } else {
                *cell = 0.0;
            }
        });

    // plant
    for particle in query.iter() {
        let index = particle.y * dimensions.width as i32 + particle.x;
        trail_map.trail[index as usize] = 100.0;
    }
}

fn angle_to_unit_vec(angle: f32) -> Vec2 {
    let radians = angle.to_radians();
    let direction_vector = Vec2::new(radians.cos(), radians.sin());
    // unit vec
    direction_vector / direction_vector.abs()
}

fn readings_3x3(vec: &[f32], index: usize, dimensions: &WorldDimensions) -> f32 {
    let mut sum = 0.0;
    // ###
    for mut x in 0..=2 {
        x -= 1;
        let i = index as i32 - x - dimensions.width;
        let line_index = ((index - index % dimensions.width as usize) / dimensions.width as usize) as i32;
        let line_i = (i - i % dimensions.width) / dimensions.width;
        if i >= 0 && line_i == line_index - 1 {
            sum += vec[i as usize];
        }
    }
    // ###
    for mut x in 0..=2 {
        x -= 1;
        let i = index as i32 - x;
        let line_index = ((index - index % dimensions.width as usize) / dimensions.width as usize) as i32;
        let line_i = (i - i % dimensions.width) / dimensions.width;
        if i >= 0 && line_i == line_index {
            sum += vec[i as usize];
        }
    }

    // ###
    for mut x in 0..=2 {
        x -= 1;
        let i = index as i32 - x + dimensions.width;
        let line_index = ((index - index % dimensions.width as usize) / dimensions.width as usize) as i32;
        let line_i = (i - i % dimensions.width) / dimensions.width;

        if i < vec.len() as i32 && line_i == line_index + 1 {
            sum += vec[i as usize];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use crate::{WorldDimensions, particles::readings_3x3};

    use super::angle_to_unit_vec;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn angle_to_unit_vec_test() {
        let angle = 45.0;
        let unit_vec = angle_to_unit_vec(angle);
        assert_eq!(unit_vec, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn mean_filter_test() {
        let vec = vec![5.0, 3.0, 6.0, 2.0, 1.0, 9.0, 8.0, 4.0, 7.0];
        let dimensions = WorldDimensions {width: 3, height: 3};
        let reading = readings_3x3(&vec, 0, &dimensions);
        assert_approx_eq!(reading, 1.22, 2f32);
    }
}
