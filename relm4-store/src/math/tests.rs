

mod common_part {
    //clippy error
    #[allow(unused_imports)]
    use crate::math::Range;

    // case 1
    #[test]
    fn other_is_to_the_left() {
        let me = Range::new(10, 20);
        let other = Range::new(5, 8);
        let result = me.common_part(&other);

        assert_eq!(result, None);
    }

    // case 2
    #[test]
    fn other_is_to_the_right() {
        let me = Range::new(10, 20);
        let other = Range::new(25, 35);
        let result = me.common_part(&other);

        assert_eq!(result, None);
    }

    // case 1
    #[test]
    fn other_is_to_the_left_border_value() {
        let me = Range::new(10, 20);
        let other = Range::new(5, 10);
        let result = me.common_part(&other);

        assert_eq!(result, None);
    }

    // case 2
    #[test]
    fn other_is_to_the_right_border_value() {
        let me = Range::new(10, 20);
        let other = Range::new(20, 35);
        let result = me.common_part(&other);

        assert_eq!(result, None);
    }

    // case 3
    #[test]
    fn other_is_in_the_middle_of_self() {
        let me = Range::new(10, 20);
        let other = Range::new(12, 18);
        let result = me.common_part(&other);

        assert_eq!(result, Some(other));
    }

    // case 3
    #[test]
    fn other_is_contained_in_self_with_start_being_equal() {
        let me = Range::new(10, 20);
        let other = Range::new(10, 18);
        let result = me.common_part(&other);

        assert_eq!(result, Some(other));
    }

    // case 3
    #[test]
    fn other_is_contained_in_self_with_end_being_equal() {
        let me = Range::new(10, 20);
        let other = Range::new(12, 20);
        let result = me.common_part(&other);

        assert_eq!(result, Some(other));
    }

    // case 3 and 4
    #[test]
    fn other_is_equal_to_self() {
        let me = Range::new(10, 20);
        let other = Range::new(10, 20);
        let result = me.common_part(&other);

        assert_eq!(result, Some(other));
        assert_eq!(result, Some(me));
    }

    // case 4
    #[test]
    fn self_is_in_the_middle_of_other() {
        let me = Range::new(12, 18);
        let other = Range::new(10, 20);
        let result = me.common_part(&other);

        assert_eq!(result, Some(me));
    }

    // case 4
    #[test]
    fn self_is_contained_in_other_with_start_being_equal() {
        let me = Range::new(10, 18);
        let other = Range::new(10, 20);
        let result = me.common_part(&other);

        assert_eq!(result, Some(me));
    }

    // case 4
    #[test]
    fn self_is_contained_in_other_with_end_being_equal() {
        let me = Range::new(12, 20);
        let other = Range::new(10, 20);
        let result = me.common_part(&other);

        assert_eq!(result, Some(me));
    }
    
    // case 5
    #[test]
    fn other_crosses_start_to_the_middle_of_self() {
        let me = Range::new(10, 20);
        let other = Range::new(8, 18);
        let result = me.common_part(&other);

        assert_eq!(result, Some(Range::new(10, 18)));
    }

    // case 5
    #[test]
    fn other_crosses_start_to_the_end_of_self() {
        let me = Range::new(10, 20);
        let other = Range::new(8, 20);
        let result = me.common_part(&other);

        assert_eq!(result, Some(Range::new(10, 20)));
    }

    // case 6
    #[test]
    fn other_crosses_end_from_the_middle_of_self() {
        let me = Range::new(10, 20);
        let other = Range::new(15, 25);
        let result = me.common_part(&other);

        assert_eq!(result, Some(Range::new(15, 20)));
    }

    // case 6
    #[test]
    fn other_crosses_end_to_the_start_of_self() {
        let me = Range::new(10, 20);
        let other = Range::new(10, 25);
        let result = me.common_part(&other);

        assert_eq!(result, Some(Range::new(10, 20)));
    }
}
