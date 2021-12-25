fn maybe_amphipod_as_str(maybe_amphipod: Option<Amphipod>) -> &'static str {
    match maybe_amphipod {
        Some(Amphipod::A) => "A",
        Some(Amphipod::B) => "B",
        Some(Amphipod::C) => "C",
        Some(Amphipod::D) => "D",
        _ => ".",
    }
}


impl<const N: usize> std::fmt::Debug for BurrowOccupancyV2<N> {
    /// Output example:
    /// #############
    /// #...........#
    /// ###B#C#B#D###
    ///   #A#D#C#A#
    ///   #########
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f);
        writeln!(f, "#############");
        write!(f, "#");
        for i in 0..11 {
            write!(f, "{}", maybe_amphipod_as_str(self.hallway[i]));
        }
        writeln!(f, "#");
        write!(f, "###");
        write!(f, "{}", maybe_amphipod_as_str(self.rooms[0][0]));
        write!(f, "#");
        write!(f, "{}", maybe_amphipod_as_str(self.rooms[1][0]));
        write!(f, "#");
        write!(f, "{}", maybe_amphipod_as_str(self.rooms[2][0]));
        write!(f, "#");
        write!(f, "{}", maybe_amphipod_as_str(self.rooms[3][0]));
        writeln!(f, "###");

        for i in 1..N {
            write!(f, "  #");
            write!(f, "{}", maybe_amphipod_as_str(self.rooms[0][i]));
            write!(f, "#");
            write!(f, "{}", maybe_amphipod_as_str(self.rooms[1][i]));
            write!(f, "#");
            write!(f, "{}", maybe_amphipod_as_str(self.rooms[2][i]));
            write!(f, "#");
            write!(f, "{}", maybe_amphipod_as_str(self.rooms[3][i]));
            writeln!(f, "#  ");
        }
        writeln!(f, "  #########  ");

        Ok(())
    }
}
