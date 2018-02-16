/// enum to return the informaiton on a collision test
#[derive(PartialEq)]
pub enum CollisionInfo {
    /// Represents the distance of separation in 3d space
    Separate3D {
        /// The overlapping distance at the front of the objects (slightly confusing used for parity
        /// with Separate 2D)
        top: f32,
        /// The overlapping distance at the back of the objects (slightly confusing used for parity
        /// with Separate 2D)
        bottom: f32,
        /// The overlapping distance at the right of the objects
        right: f32,
        /// The overlapping distance at the left of the objects
        left: f32,
        /// The overlapping distance above the objects
        up: f32,
        /// The overlapping distance below the objects
        down: f32,
    },
    /// Represents the distance of separation in 2d space
    Separate2D {
        /// The overlapping distance at the top of the objects
        top: f32,
        /// The overlapping distance at the bottom of the objects
        bottom: f32,
        /// The overlapping distance at the left of the objects
        right: f32,
        /// The overlapping distance at the right of the objects
        left: f32,
    },
    /// Used for if two objects are overlapping each other
    Overlapping,
    /// When no collision has taken place
    NoCollision,
}

/// returns true if the given squares with width are overlapping
pub fn overlapping(
    pos_a: &(f32, f32),
    size_a: &(f32, f32),
    pos_b: &(f32, f32),
    size_b: &(f32, f32),
    separate: bool,
) -> CollisionInfo {
    let (left_a, left_b) = (pos_a.0, pos_b.0);
    let (right_a, right_b) = (pos_a.0 + size_a.0, pos_b.0 + size_b.0);
    let (top_a, top_b) = (pos_a.1, pos_b.1);
    let (bottom_a, bottom_b) = (pos_a.1 + size_a.1, pos_b.1 + size_b.1);

    if bottom_a <= top_b {
        return CollisionInfo::NoCollision;
    }
    if top_a >= bottom_b {
        return CollisionInfo::NoCollision;
    }
    if right_a <= left_b {
        return CollisionInfo::NoCollision;
    }
    if left_a >= right_b {
        return CollisionInfo::NoCollision;
    }

    if separate {
        let bottom_diff = (bottom_a - top_b).abs();
        let top_diff = (top_a - bottom_b).abs();
        let right_diff = (right_a - left_b).abs();
        let left_diff = (left_a - right_b).abs();

        return CollisionInfo::Separate2D {
            top: top_diff,
            bottom: bottom_diff,
            right: right_diff,
            left: left_diff,
        };
    }

    CollisionInfo::Overlapping
}

/// returns true if the given cubes with size overlap
pub fn overlapping_3d(
    pos_a: &(f32, f32, f32),
    size_a: &(f32, f32, f32),
    pos_b: &(f32, f32, f32),
    size_b: &(f32, f32, f32),
    separate: bool,
) -> CollisionInfo {
    // check collision in 2d
    let check_2d = overlapping(
        &(pos_a.0, pos_a.2),
        &(size_a.0, size_a.2),
        &(pos_b.0, pos_b.2),
        &(size_b.0, size_b.2),
        separate,
    );
    // return if there is no collision in 2d
    if let CollisionInfo::NoCollision = check_2d {
        return check_2d;
    }

    let (up_a, up_b) = (pos_a.1, pos_b.1);
    let (down_a, down_b) = (pos_a.1 + size_a.1, pos_b.1 + size_b.1);

    if down_a <= up_b {
        return CollisionInfo::NoCollision;
    }
    if up_a >= down_b {
        return CollisionInfo::NoCollision;
    }

    if let CollisionInfo::Separate2D {
        top: t,
        bottom: b,
        right: r,
        left: l,
    } = check_2d
    {
        let up_diff = (up_a - down_b).abs();
        let down_diff = (down_a - up_b).abs();

        return CollisionInfo::Separate3D {
            top: t,
            bottom: b,
            right: r,
            left: l,
            up: up_diff,
            down: down_diff,
        };
    }

    CollisionInfo::Overlapping
}
