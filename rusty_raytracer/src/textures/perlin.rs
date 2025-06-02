// translated directly from the cpp in listing 34 of ray tracing the next week
// the tutorial gave no explanation of the code

use nalgebra::{Point3, Vector3};

use crate::{util::vector_math::{random_unit_vector, random_f32_within}};

pub struct Perlin {
    point_count: usize, // 256;
    randvec: [Vector3<f32>; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {

    pub fn new() -> Self {
        let mut perlin = Self {
            point_count: 256,
            randvec: [Vector3::zeros(); 256],
            perm_x: [0; 256],
            perm_y: [0; 256],
            perm_z: [0; 256],
        };
        perlin.init();
        return perlin;
    }

    pub fn init(&mut self) {

        for i in 0..self.point_count {
            self.randvec[i] = random_unit_vector();
        }

        Perlin::perlin_generate_perm(&mut self.perm_x, self.point_count);
        Perlin::perlin_generate_perm(&mut self.perm_y, self.point_count);
        Perlin::perlin_generate_perm(&mut self.perm_z, self.point_count);
    }

    pub fn noise(&self, p: &Point3<f32>) -> f32 {

        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        /*
        // hermitian smoothing so that the interpolation doesn't look too grid-like
        u = u*u*(3.0-2.0*u);
        v = v*v*(3.0-2.0*v);
        w = w*w*(3.0-2.0*w);
        */

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[Vector3<f32>; 2]; 2]; 2] = [[[Vector3::zeros(); 2]; 2]; 2];

        // the typing is a mess in this loop... fix at some point
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = (self.perm_x[((i+di) & 255) as usize] ^ self.perm_y[((j+dj) & 255) as usize] ^ self.perm_z[((k+dk) & 255) as usize]) as usize;
                    c[di as usize][dj as usize][dk as usize] = self.randvec[idx];
                }
            }
        }

        //return self.trilinear_interpolation(c, u, v, w);
        return self.perlin_interpolation(c, u, v, w);

        /*without linear interpolation 
        let i = ((4.0*p.x) as i32 & 255) as usize;
        let j = ((4.0*p.y) as i32 & 255) as usize;
        let k = ((4.0*p.z) as i32 & 255) as usize;
        let idx = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        return self.randf32[idx];
        */
    }

    fn perlin_generate_perm(p: &mut [i32; 256], n: usize) {
        for i in 0..n {
            p[i] = i as i32;
        }

        for i in (1..n).rev() {
            // is there a nice rust function that does this? swap
            let r = random_f32_within(0.0, (i+1) as f32);
            let j = r.floor() as usize;
            let tmp = p[i as usize];
            p[i as usize] = p[j];
            p[j] = tmp;
        }
    }

    fn trilinear_interpolation(&self, c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;
        // the typing is a mess in this loop... fix at some point
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let di = i as f32;
                    let dj = j as f32;
                    let dk = k as f32;
                    accum += (di*u + (1.0-di)*(1.0-u))
                           * (dj*v + (1.0-dj)*(1.0-v))
                           * (dk*w + (1.0-dk)*(1.0-w))
                           * c[i][j][k];    
                }
            }
        }
        return accum;
    }

    fn perlin_interpolation(&self, c: [[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        // hermitian smoothing here instead now
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;

        // the typing is a mess in this loop... fix at some point
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let di = i as f32;
                    let dj = j as f32;
                    let dk = k as f32;
                    let weight_v = Vector3::new(u-di, v-dj, w-dk);
                    accum += (di*uu + (1.0-di)*(1.0-uu))
                           * (dj*vv + (1.0-dj)*(1.0-vv))
                           * (dk*ww + (1.0-dk)*(1.0-ww))
                           * weight_v.dot(&c[i][j][k]);    
                }
            }
        }
        return accum;
        
    }

    pub fn turbulance(&self, p: &mut Point3<f32>, depth: u32) -> f32 {
        let mut accum = 0.0;
        let temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight*self.noise(temp_p);
            weight *= 0.5;
            temp_p.x *= 2.0;
            temp_p.y *= 2.0;
            temp_p.z *= 2.0;
        }
        return accum.abs();
    }

    fn clone_box(&self) -> Box<dyn Send + Sync> {return Box::new(self.clone());}

}


impl Clone for Perlin {
    fn clone(&self) -> Self {
        Self {
            point_count: self.point_count, // 256;
            randvec: self.randvec,
            perm_x: self.perm_x,
            perm_y: self.perm_y,
            perm_z: self.perm_z,
        }
    }
}