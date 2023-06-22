use ambient_api::prelude::*;
use crate::drawing;

//#[test]
fn color_test(color: Vec4) {
	let color_lower_limit = 0.;
	let color_higher_limit = 1.;
	more_asserts::debug_assert_ge!(color.x, color_lower_limit, "X value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_ge!(color.y, color_lower_limit, "Y value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_ge!(color.z, color_lower_limit, "Z value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_ge!(color.w, color_lower_limit, "W value of color is smaller than {}", color_lower_limit);
	more_asserts::debug_assert_le!(color.x, color_higher_limit, "X value of color is bigger than {}", color_higher_limit);
	more_asserts::debug_assert_le!(color.y, color_higher_limit, "Y value of color is bigger than {}", color_higher_limit);
	more_asserts::debug_assert_le!(color.z, color_higher_limit, "Z value of color is bigger than {}", color_higher_limit);
	more_asserts::debug_assert_le!(color.w, color_higher_limit, "W value of color is bigger than {}", color_higher_limit);
}

fn clock_border_color_test(){
	color_test(drawing::CLOCK_BORDER_COLOR);
}

fn hour_color_test(){
	color_test(drawing::HOUR_COLOR);
}

fn minute_color_test(){
	color_test(drawing::MINUTE_COLOR);
}

fn second_color_test(){
	color_test(drawing::SECOND_COLOR);
}

//the etc_color variables are Vec4 used for the clock colors
pub fn color_tests(){
	clock_border_color_test();
	hour_color_test();
	minute_color_test();
	second_color_test();
}