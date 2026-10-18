fn main() -> anyhow::Result<()> {
    boardgame_game::connect4_tui::run_tui(None)
}
