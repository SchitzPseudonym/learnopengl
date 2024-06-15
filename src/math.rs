use std::ops::{self, Mul};

pub struct Vec3 { x: f32, y: f32, z: f32 }
impl Vec3
{
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { return Vec3 { x: x, y: y, z: z } }
    pub fn uniform(u: f32) -> Vec3 { return Vec3 { x: u, y: u, z: u } }
    pub fn magnitude(&self) -> f32 { return (self.x + self.y + self.z).sqrt(); }
    pub fn normalized(&self) -> Vec3 { let mag: f32 = self.magnitude(); return Vec3 { x: self.x/mag, y: self.y/mag, z: self.z/mag } }
}
impl ops::Add for Vec3 //only traits defined in the current crate can be implemented for arbitrary types. define and implement a trait or new type instead
{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 { return Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }; }
}

#[derive(Copy, Clone)]
pub struct Mat4 { pub data: [[f32; 4]; 4] }
impl Mat4
{
    pub fn new(scale: f32) -> Mat4 { Mat4{ data: [[scale, 0.0, 0.0, 0.0], [0.0, scale, 0.0, 0.0], [0.0, 0.0, scale, 0.0], [0.0, 0.0, 0.0, scale]]} }
    #[deprecated] #[allow(dead_code)]
    pub fn rotation(theta: f32, axis: Vec3) -> Mat4 {
        let do_x = {axis.x > 0.0};
        let do_y = {axis.y > 0.0};
        let do_z = {axis.z > 0.0};

        let (sin, cos) = theta.sin_cos();

        return Mat4 { data: [
            [{if do_y || do_z { cos } else { 1.0 }}, {if do_z { -sin } else { 0.0 }}       , {if do_y { sin } else { 0.0 }}        , 0.0],
            [{if do_z { sin } else { 0.0 }}        , {if do_x || do_z { cos } else { 1.0 }}, {if do_x { -sin } else { 0.0 }}       , 0.0],
            [{if do_y { -sin } else { 0.0 }}       , {if do_x { sin } else { 0.0 }}        , {if do_x || do_y { cos } else { 1.0 }}, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]};
    }
    #[deprecated] #[allow(dead_code)]
    pub fn scalar_vec3(scale: Vec3) -> Mat4
    {
        return Mat4 { data: [
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]};
    }
    #[deprecated] #[allow(dead_code)]
    pub fn translation(translation: Vec3) -> Mat4
    {
        return Mat4 { data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [translation.x, translation.y, translation.z, 1.0],
        ]};
    }
    //Rotation * Scalar * Translation
    pub fn transform(theta: f32, axis: Vec3, scale: Vec3, translation: Vec3) -> Mat4
    {
        let do_x = {axis.x > 0.0};
        let do_y = {axis.y > 0.0};
        let do_z = {axis.z > 0.0};

        let (sin, cos) = theta.sin_cos();
        
        return Mat4 {data:[
            [ {if do_y || do_z { cos } else { 1.0 }}*scale.x, {if do_z { -sin } else { 0.0 }}*scale.y, {if do_y { sin } else { 0.0 }}*scale.z, 0.0, ],
            [ {if do_z { sin } else { 0.0 }}*scale.x, {if do_x || do_z { cos } else { 1.0 }}*scale.y, {if do_x { -sin } else { 0.0 }}*scale.z, 0.0, ],
            [ {if do_y { -sin } else { 0.0 }}*scale.x, {if do_x { sin } else { 0.0 }}*scale.y, {if do_x || do_y { cos } else { 1.0 }}*scale.z, 0.0, ],
            [ translation.x, translation.y, translation.z, 1.0, ],
        ]};
    }
    //Look into right handed vs left handed coordinate systems
    pub fn orthographic(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4
    {
        return Mat4 {data: [
            [1.0/(right-left), 0.0,              0.0,            (right+left)/2.0],
            [0.0,              1.0/(bottom-top), 0.0,            (bottom+top)/2.0],
            [0.0,              0.0,              1.0/(far-near), near],
            [0.0,              0.0,              0.0,            1.0],
        ]};
    }
    pub fn perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4
    {
        return Mat4 {data: [
            [znear, 0.0, 0.0, 0.0],
            [0.0, znear, 0.0, 0.0],
            [0.0, 0.0, (zfar+znear), -zfar*znear],
            [0.0, 0.0, 1.0, 0.0],
        ]}
        // let f = 1.0 / (fov/2.0).tan();
        // return Mat4 {data:[
        //     [f/aspect, 0.0, 0.0, 0.0],
        //     [0.0, f, 0.0, 0.0],
        //     [0.0, 0.0, (zfar+znear)/(znear-zfar), (2.0*zfar*znear)/(znear-zfar)],
        //     [0.0, 0.0, -1.0, 0.0],
        // ]}
    }
}
impl Mul for Mat4
{
    type Output = Mat4;
    // fn mul(self, rhs: Mat4) -> Mat4 {return Mat4 { data: [
    //     [self.data[0][0] + rhs.data[0][0], self.data[0][1] + rhs.data[0][1], self.data[0][2] + rhs.data[0][2], self.data[0][3] + rhs.data[0][3]],
    //     [self.data[1][0] + rhs.data[1][0], self.data[1][1] + rhs.data[1][1], self.data[1][2] + rhs.data[1][2], self.data[1][3] + rhs.data[1][3]],
    //     [self.data[2][0] + rhs.data[2][0], self.data[2][1] + rhs.data[2][1], self.data[2][2] + rhs.data[2][2], self.data[2][3] + rhs.data[2][3]],
    //     [self.data[3][0] + rhs.data[3][0], self.data[3][1] + rhs.data[3][1], self.data[3][2] + rhs.data[3][2], self.data[3][3] + rhs.data[3][3]],
    // ]}}
    fn mul(self, rhs: Mat4) -> Mat4
    {
        let ldata = self.data;
        let rdata = rhs.data;
        return Mat4 {
            data: [
                [
                    (ldata[0][0]*rdata[0][0]) + (ldata[0][1]*rdata[1][0]) + (ldata[0][2]*rdata[2][0]) + (ldata[0][3]*rdata[3][0]),
                    (ldata[0][0]*rdata[0][1]) + (ldata[0][1]*rdata[1][1]) + (ldata[0][2]*rdata[2][1]) + (ldata[0][3]*rdata[3][1]),
                    (ldata[0][0]*rdata[0][2]) + (ldata[0][1]*rdata[1][2]) + (ldata[0][2]*rdata[2][2]) + (ldata[0][3]*rdata[3][2]),
                    (ldata[0][0]*rdata[0][3]) + (ldata[0][1]*rdata[1][3]) + (ldata[0][2]*rdata[2][3]) + (ldata[0][3]*rdata[3][3]),
                ],
                [
                    (ldata[1][0]*rdata[0][0]) + (ldata[1][1]*rdata[1][0]) + (ldata[1][2]*rdata[2][0]) + (ldata[1][3]*rdata[3][0]),
                    (ldata[1][0]*rdata[0][1]) + (ldata[1][1]*rdata[1][1]) + (ldata[1][2]*rdata[2][1]) + (ldata[1][3]*rdata[3][1]),
                    (ldata[1][0]*rdata[0][2]) + (ldata[1][1]*rdata[1][2]) + (ldata[1][2]*rdata[2][2]) + (ldata[1][3]*rdata[3][2]),
                    (ldata[1][0]*rdata[0][3]) + (ldata[1][1]*rdata[1][3]) + (ldata[1][2]*rdata[2][3]) + (ldata[1][3]*rdata[3][3]),
                ],
                [
                    (ldata[2][0]*rdata[0][0]) + (ldata[2][1]*rdata[1][0]) + (ldata[2][2]*rdata[2][0]) + (ldata[2][3]*rdata[3][0]),
                    (ldata[2][0]*rdata[0][1]) + (ldata[2][1]*rdata[1][1]) + (ldata[2][2]*rdata[2][1]) + (ldata[2][3]*rdata[3][1]),
                    (ldata[2][0]*rdata[0][2]) + (ldata[2][1]*rdata[1][2]) + (ldata[2][2]*rdata[2][2]) + (ldata[2][3]*rdata[3][2]),
                    (ldata[2][0]*rdata[0][3]) + (ldata[2][1]*rdata[1][3]) + (ldata[2][2]*rdata[2][3]) + (ldata[2][3]*rdata[3][3]),
                ],
                [
                    (ldata[3][0]*rdata[0][0]) + (ldata[3][1]*rdata[1][0]) + (ldata[3][2]*rdata[2][0]) + (ldata[3][3]*rdata[3][0]),
                    (ldata[3][0]*rdata[0][1]) + (ldata[3][1]*rdata[1][1]) + (ldata[3][2]*rdata[2][1]) + (ldata[3][3]*rdata[3][1]),
                    (ldata[3][0]*rdata[0][2]) + (ldata[3][1]*rdata[1][2]) + (ldata[3][2]*rdata[2][2]) + (ldata[3][3]*rdata[3][2]),
                    (ldata[3][0]*rdata[0][3]) + (ldata[3][1]*rdata[1][3]) + (ldata[3][2]*rdata[2][3]) + (ldata[3][3]*rdata[3][3]),
                ],
            ]
        }
    }
}