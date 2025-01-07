mod constant;
mod types;
mod util;
mod grid2d;

#[cfg(test)]
mod tests {
    use super::types::Grid;
    use super::grid2d::Grid2D;
    #[test]
    fn it_works() {
        let grid = Grid2D::new(118.0,32.0,4326,1.0,-1.0,10,10,vec![10.0;100]);
        let val: f64 = grid.get_val(120.0, 26.0);
        assert_eq!(val, 10.0);
        assert_eq!(grid.get_invalid(), -99999999.0);
        assert_eq!(grid.get_end_x(), 128.0);
        assert_eq!(grid.get_end_y(), 22.0);
    }
}
