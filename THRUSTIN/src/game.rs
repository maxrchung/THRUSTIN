fn gameloop(lobby: lobby::Lobby) {
    let mut points: std::vec::Vec<u32> = Vec::with_capacity(lobby.count as usize);

    while {
        //reveal thrust

        let thrustee: usize;
        //put in your thrusters
        for (i, p) in &lobby.list.iter().enumerate() {
            if p.state == thruster {
                //put in that thrust boi
            } else if p.state == thruster {
                thrustee = i;
            }
        }

        //thrustee picks a thruster
        
    }
}
