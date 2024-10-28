use crate::prelude::*;
use crate::satellite::OrbitalElements;

use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// Creates a colorful test pattern
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

use serde::{Deserialize, Serialize};
/// Example of SatelliteData:
/// {
///     "OBJECT_NAME":"STARLINK-1008",
///     "OBJECT_ID":"2019-074B",
///     "EPOCH":"2024-10-27T04:10:58.101312",
///     "MEAN_MOTION":15.06400535,
///     "ECCENTRICITY":0.0001539,
///     "INCLINATION":53.0535,
///     "RA_OF_ASC_NODE":264.5418,
///     "ARG_OF_PERICENTER":101.0513,
///     "MEAN_ANOMALY":259.0649,
///     "EPHEMERIS_TYPE":0,
///     "CLASSIFICATION_TYPE":"U",
///     "NORAD_CAT_ID":44714,
///     "ELEMENT_SET_NO":999,
///     "REV_AT_EPOCH":27361,
///     "BSTAR":0.00030439,
///     "MEAN_MOTION_DOT":4.255e-5,
///     "MEAN_MOTION_DDOT":0
/// }
#[derive(Serialize, Deserialize, Debug)]
pub struct SatelliteData {
    MEAN_MOTION: f32, // (rev/day)
    ECCENTRICITY: f32,
    INCLINATION: f32,
    RA_OF_ASC_NODE: f32,
    ARG_OF_PERICENTER: f32,
    MEAN_ANOMALY: f32,
}

const FACTOR: f32 = 42241.09567; // u^(1/3) * (2 pi / 86400) ^ (-2/3)
impl SatelliteData {
    pub fn to_orbital_elements(&self) -> OrbitalElements {
        let u = 398600.4418; // Earth's gravitational constant in km^3/s^
        let n = self.MEAN_MOTION.powf(-2. / 3.);
        let semi_major_axis = FACTOR * n;
        let eccentricity = self.ECCENTRICITY;
        let inclination = self.INCLINATION;
        let argument_of_periapsis = self.ARG_OF_PERICENTER;
        let longitude_of_ascending_node = self.RA_OF_ASC_NODE;
        OrbitalElements::new(
            semi_major_axis,
            eccentricity,
            inclination,
            argument_of_periapsis,
            longitude_of_ascending_node,
            true_anomaly,
        )
    }
}
