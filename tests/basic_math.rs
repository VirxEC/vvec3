use vvec3::Vec3;

#[test]
fn add() {
    let vec = Vec3::new(1., 1., 0.);
    let vec2 = vec + Vec3::new(2., 5.2, 0.5);

    assert!(vec2.x == 3.);
    assert!(vec2.y == 6.2);
    assert!(vec2.z == 0.5);
    
    let vec3 = vec + 3.;

    assert!(vec3.x == 4.);
    assert!(vec3.y == 4.);
    assert!(vec3.z == 3.);
    
    let vec4 = 3. + vec;

    assert!(vec4.x == 4.);
    assert!(vec4.y == 4.);
    assert!(vec4.z == 3.);
}

#[test]
fn add_assign() {
    let mut vec = Vec3::new(1., 1., 0.);
    vec += Vec3::new(2., 5.2, 0.5);

    assert!(vec.x == 3.);
    assert!(vec.y == 6.2);
    assert!(vec.z == 0.5);

    let mut vec2 = Vec3::new(1., 1., 0.);
    vec2 += 3.;

    assert!(vec2.x == 4.);
    assert!(vec2.y == 4.);
    assert!(vec2.z == 3.);
}

#[test]
fn sub() {
    let vec = Vec3::new(1., 1., 0.);
    let vec2 = vec - Vec3::new(2., 5.2, 0.5);

    assert!(vec2.x == -1.);
    assert!(vec2.y == -4.2);
    assert!(vec2.z == -0.5);
    
    let vec3 = vec - 3.;

    assert!(vec3.x == -2.);
    assert!(vec3.y == -2.);
    assert!(vec3.z == -3.);
    
    let vec4 = 3. - vec;

    assert!(vec4.x == 2.);
    assert!(vec4.y == 2.);
    assert!(vec4.z == 3.);
}

#[test]
fn sub_assign() {
    let mut vec = Vec3::new(1., 1., 0.);
    vec -= Vec3::new(2., 5.2, 0.5);

    assert!(vec.x == -1.);
    assert!(vec.y == -4.2);
    assert!(vec.z == -0.5);
    
    let mut vec2 = Vec3::new(1., 1., 0.);
    vec2 -= 3.;

    assert!(vec2.x == -2.);
    assert!(vec2.y == -2.);
    assert!(vec2.z == -3.);
}

#[test]
fn mul() {
    let vec = Vec3::new(1., 1., 0.);
    let vec2 = vec * Vec3::new(2., 5.2, 0.5);

    assert!(vec2.x == 2.);
    assert!(vec2.y == 5.2);
    assert!(vec2.z == 0.);
    
    let vec3 = vec * 3.;

    assert!(vec3.x == 3.);
    assert!(vec3.y == 3.);
    assert!(vec3.z == 0.);
    
    let vec4 = 3. * vec;

    assert!(vec4.x == 3.);
    assert!(vec4.y == 3.);
    assert!(vec4.z == 0.);
}

#[test]
fn mul_assign() {
    let mut vec = Vec3::new(1., 1., 0.);
    vec *= Vec3::new(2., 5.2, 0.5);

    assert!(vec.x == 2.);
    assert!(vec.y == 5.2);
    assert!(vec.z == 0.);

    let mut vec2 = Vec3::new(1., 1., 0.);
    vec2 *= 3.;

    assert!(vec2.x == 3.);
    assert!(vec2.y == 3.);
    assert!(vec2.z == 0.);
}

#[test]
fn div() {
    let vec = Vec3::new(1., 1., 0.);
    let vec2 = vec / Vec3::new(2., 5.2, 0.5);

    assert!(vec2.x == 0.5);
    assert!(vec2.y == 1./5.2);
    assert!(vec2.z == 0.);
    
    let vec3 = vec / 3.;

    assert!(vec3.x == 1./3.);
    assert!(vec3.y == 1./3.);
    assert!(vec3.z == 0.);
    
    let vec3 = 3. / vec;

    assert!(vec3.x == 3.);
    assert!(vec3.y == 3.);
}

#[test]
fn div_assign() {
    let mut vec = Vec3::new(1., 1., 0.);
    vec /= Vec3::new(2., 5.2, 0.5);

    assert!(vec.x == 0.5);
    assert!(vec.y == 1./5.2);
    assert!(vec.z == 0.);
    
    let mut vec2 = Vec3::new(1., 1., 0.);
    vec2 /= 3.;

    assert!(vec2.x == 1./3.);
    assert!(vec2.y == 1./3.);
    assert!(vec2.z == 0.);
}
