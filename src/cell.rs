#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
	Space,
	NormalBarrier,
	SpecialBarrier(char),
	Goal(char),
	Start(char),
	WarpInner(u16),
	WarpOuter(u16),
	Items
}

impl CellType {}
