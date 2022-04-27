pub fn xyz_to_lab_entries (buf:&[u8],entries: usize,mode: &XYZtoLabCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let wp = mode.get();
    let index = (entries / 3) as usize;
    let mut buffer = Vec::with_capacity(index * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let x = buf[ptr]     as f32 / 255.0;
        let y = buf[ptr + 1] as f32 / 255.0;
        let z = buf[ptr + 2] as f32 / 255.0;
        let th = 0.008856;

        let x_3 = if x / wp.x > th { (x / wp.x).powf(1.0/3.0) } else { 7.78 * x / wp.x + 16.0 / 116.0};
        let y_3 = if y / wp.y > th { (y / wp.y).powf(1.0/3.0) } else { 7.78 * y / wp.y + 16.0 / 116.0};
        let z_3 = if z / wp.z > th { (z / wp.z).powf(1.0/3.0) } else { 7.78 * z / wp.z + 16.0 / 116.0};

        let l = 116.0 * y_3 - 16.0;
        let a = 500.0 * (x_3 - y_3);
        let b = 200.0 * (y_3 - z-3);
        let l = (l / 100.0 * 255.0) as i16).clamp(0,255) as u8;
        let a = (a + 128.0) as i16).clamp(0,255) as u8;
        let b = (b + 128.0) as i16).clamp(0,255) as u8;

        buffer.push(l);
        buffer.push(a);
        buffer.push(b);
    }

    Ok(buffer)
}

pub fn xyz_to_lab_entries_u16 (buf:&[u8],entries: usize,mode: &XYZtoLabCoefficient) -> Result<Vec<u16>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let wp = mode.get();
    let index = (entries / 3) as usize;
    let mut buffer = Vec::with_capacity(index * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let x = buf[ptr]     as f32 / 255.0;
        let y = buf[ptr + 1] as f32 / 255.0;
        let z = buf[ptr + 2] as f32 / 255.0;
        let th = 0.008856;

        let x_3 = if x / wp.x > th { (x / wp.x).powf(1.0/3.0) } else { 7.78 * x / wp.x + 16.0 / 116.0};
        let y_3 = if y / wp.y > th { (y / wp.y).powf(1.0/3.0) } else { 7.78 * y / wp.y + 16.0 / 116.0};
        let z_3 = if z / wp.z > th { (z / wp.z).powf(1.0/3.0) } else { 7.78 * z / wp.z + 16.0 / 116.0};

        let l = 116.0 * y_3 - 16.0;
        let a = 500.0 * (x_3 - y_3);
        let b = 200.0 * (y_3 - z-3);
        let l = (l / 100.0 * 65535.0) as i32).clamp(0,65535) as u16;
        let a = ((a + 128.0) * 255.0) as i32).clamp(0,65535) as u16;
        let b = ((b + 128.0) * 255.0) as i32).clamp(0,65535) as u16;

        buffer.push(l);
        buffer.push(a);
        buffer.push(b);
    }

    Ok(buffer)
}
