pub fn transration_prametic_curve(buf:&[u8] ,entry:usize,prametic_curve:ParametricCurve) -> Result<Vec<u8>>{
    if buf.len() < entry {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut p = vec![];
    for val in prametic_curve.vals {
        p.push(val.as_f64())
    }
    let mut data = vec![];
 
    for i in 0..entry {
        let x = buf[i] as f64;
        let y;
        match prametic_curve.funtion_type {
            0x000 => {
                let ganma = p[0];
                y = x.powf(ganma); 
            },
            0x001 => {
                let ganma = p[0];
                let a = p[1];
                let b = p[2];
                y = if x >= - b / a {
                    (a * x + b).powf(gamma);
                } else {
                    0.0
                };
            },
            0x002 => {
                let ganma = p[0];
                let a = p[1];
                let b = p[2];
                let c = p[3];
                y = if x >= - b / a {
                    (a * x + b).powf(gamma) + c;
                } else {
                    c
                };
            },
            0x003 => {
                let ganma = p[0];
                let a = p[1];
                let b = p[2];
                let c = p[3];
                let d = p[4];
                let y = if x >= d {
                    (a * x + b).powf(gamma);
                } else {
                    c * x
                };
            },
            0x004 => {
                let ganma = p[0];
                let a = p[1];
                let b = p[2];
                let c = p[3];
                let d = p[4];
                let e = p[5];
                let f = p[6];
                let y = if x >= d {
                    (a * x + b).powf(gamma + e);
                } else {
                    c * x + f;
                };
            _ => { y = x},
        }
        data.push((y as i16 + 0.5).clamp(0,255) as u8);
    }
    Ok(data)
}

