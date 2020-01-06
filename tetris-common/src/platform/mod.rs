pub fn transform_symbol(symbol: &str) -> String {
	if cfg!(target_os = "windows") {
		if " ".eq(symbol) {
			return symbol.to_owned() + " ";
		}
		return symbol.to_owned();
	}
	return symbol.to_owned() + " ";
}