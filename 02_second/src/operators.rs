use crate::foundations::*;

pub struct Position<Object>
where Object: DistanceField
{ // recommended : top of operation tree
    object: Object,
    position: Vector3
}

pub struct Rolled<Object>
where Object: DistanceField
{
    object: Object,
    center: Vector3,
    axis: Vector3,
    angle: f64
}

pub struct Textured<Object>
where Object: DistanceField + UVMap {
    object: Object,
    texture: ImageBuffer<Rgba<u8>, Vec<u8>>
}

pub struct Bumped<Object>
where Object: DistanceField + UVMap {
    object: Object,
    bumpmap: BumpMap
}

pub struct BumpMap {
    map : Vec<u8>,
    width: usize,
    height: usize
}

impl<Object> DistanceField for Position<Object>
where Object : DistanceField
{
    fn distance(&self, point: Vector3) -> f64 {
        self.object.distance(point - self.position)
    }
}

impl<Object> DistanceField for Rolled<Object>
where Object : DistanceField
{
    fn distance(&self, point: Vector3) -> f64 {
    }
}