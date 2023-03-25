
#[derive(Debug, Clone, PartialEq)]
pub struct RGBAColor(pub f64, pub f64, pub f64, pub f64);

pub fn over_operator(pixel1: &RGBAColor, pixel2: &RGBAColor) -> RGBAColor {
    let alpha1 = pixel1.3 / 255.0;
    let alpha2 = pixel2.3 / 255.0;

    let second_alpha_component = alpha2*(1.0-alpha1);
    let new_alpha = alpha1 + second_alpha_component;

    let new_color_r = (pixel1.0*alpha1 + pixel2.0*alpha2*second_alpha_component)/new_alpha;
    let new_color_g = (pixel1.1*alpha1 + pixel2.1*alpha2*second_alpha_component)/new_alpha;
    let new_color_b = (pixel1.2*alpha1 + pixel2.2*alpha2*second_alpha_component)/new_alpha;

    RGBAColor(new_color_r, new_color_g, new_color_b, new_alpha*255.0)
}

pub const RED: RGBAColor = RGBAColor(255.0, 0.0, 0.0, 0.0);
pub const GREEN: RGBAColor = RGBAColor(0.0, 255.0, 0.0, 0.0);
pub const BLUE: RGBAColor = RGBAColor(0.0, 0.0, 255.0, 0.0);
pub const WHITE: RGBAColor = RGBAColor(255.0, 255.0, 255.0, 0.0);
pub const BLACK: RGBAColor = RGBAColor(0.0, 0.0, 0.0, 0.0);
pub const YELLOW: RGBAColor = RGBAColor(255.0, 255.0, 0.0, 0.0);
pub const CYAN: RGBAColor = RGBAColor(0.0, 255.0, 255.0, 0.0);
pub const MAGENTA: RGBAColor = RGBAColor(255.0, 0.0, 255.0, 0.0);
pub const SILVER: RGBAColor = RGBAColor(192.0, 192.0, 192.0, 0.0);
pub const GREY: RGBAColor = RGBAColor(128.0, 128.0, 128.0, 0.0);
pub const DARK_RED: RGBAColor = RGBAColor(128.0, 0.0, 0.0, 0.0);
pub const DARK_GREEN: RGBAColor = RGBAColor(0.0, 128.0, 0.0, 0.0);
pub const DARK_BLUE: RGBAColor = RGBAColor(0.0, 0.0, 128.0, 0.0);
pub const ORANGE: RGBAColor = RGBAColor(255.0, 165.0, 0.0, 0.0);
pub const PURPLE: RGBAColor = RGBAColor(128.0, 0.0, 128.0, 0.0);
pub const LIGHT_GREY: RGBAColor = RGBAColor(211.0, 211.0, 211.0, 0.0);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn over_operator_test() {
        let pixel1 = RGBAColor(200.0, 100.0, 50.0, 127.5);
        let pixel2 = RGBAColor(50.0, 250.0, 0.0, 255.0);

        let new_pixel = over_operator(&pixel1, &pixel2);
        assert_eq!(new_pixel, RGBAColor(125.0, 175.0, 25.0, 255.0));


        let pixel1 = RGBAColor(200.0, 100.0, 50.0, 255.0);
        let pixel2 = RGBAColor(50.0, 250.0, 0.0, 0.0);

        let new_pixel = over_operator(&pixel1, &pixel2);
        assert_eq!(new_pixel, RGBAColor(200.0, 100.0, 50.0, 255.0));
    }
}
