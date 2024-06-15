use std::collections::BTreeMap;

#[derive(Default)]
struct Stat {
    mp: u32, // Matches Played
    w: u32,  // Matches Won
    d: u32,  // Matches Drawn (Tied)
    l: u32,  // Matches Lost
    p: u32,  // Points
}

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

/// # Panics
///
/// Will panic if match result isn't following the following pattern: `team1;team2;result`
/// where `result` is either `win`, `draw` or `loss`
#[must_use]
pub fn tally(match_results: &str) -> String {
    let mut tournament_result = HEADER.to_string();
    if match_results.is_empty() {
        return tournament_result;
    }
    let mut map: BTreeMap<&str, Stat> = BTreeMap::new();
    let match_results: Vec<_> = match_results.split('\n').collect();
    for x in &match_results {
        let v: Vec<_> = x.split(';').collect();
        match v.get(2) {
            Some(&"win") => {
                map.entry(v[0])
                    .and_modify(|stat| {
                        stat.mp += 1;
                        stat.w += 1;
                        stat.p += 3;
                    })
                    .or_insert(Stat {
                        mp: 1,
                        w: 1,
                        p: 3,
                        ..Default::default()
                    });
                map.entry(v[1])
                    .and_modify(|stat| {
                        stat.mp += 1;
                        stat.l += 1;
                    })
                    .or_insert(Stat {
                        mp: 1,
                        l: 1,
                        ..Default::default()
                    });
            }
            Some(&"draw") => {
                map.entry(v[0])
                    .and_modify(|stat| {
                        stat.mp += 1;
                        stat.d += 1;
                        stat.p += 1;
                    })
                    .or_insert(Stat {
                        mp: 1,
                        d: 1,
                        p: 1,
                        ..Default::default()
                    });
                map.entry(v[1])
                    .and_modify(|stat| {
                        stat.mp += 1;
                        stat.d += 1;
                        stat.p += 1;
                    })
                    .or_insert(Stat {
                        mp: 1,
                        d: 1,
                        p: 1,
                        ..Default::default()
                    });
            }
            Some(&"loss") => {
                map.entry(v[1])
                    .and_modify(|stat| {
                        stat.mp += 1;
                        stat.w += 1;
                        stat.p += 3;
                    })
                    .or_insert(Stat {
                        mp: 1,
                        w: 1,
                        p: 3,
                        ..Default::default()
                    });
                map.entry(v[0])
                    .and_modify(|stat| {
                        stat.mp += 1;
                        stat.l += 1;
                    })
                    .or_insert(Stat {
                        mp: 1,
                        l: 1,
                        ..Default::default()
                    });
            }
            _ => panic!("Match formatting isn't correct"),
        }
    }
    let mut list: Vec<_> = map.iter().collect();
    list.sort_by(|(_, Stat { p: l, .. }), (_, Stat { p: r, .. })| r.cmp(l));
    for (team, stat) in &list {
        tournament_result.push('\n');
        tournament_result.push_str(team);
        for _ in 0..31 - team.len() {
            tournament_result.push(' ');
        }
        tournament_result.push_str(&format!(
            "|  {} |  {} |  {} |  {} |  {}",
            stat.mp, stat.w, stat.d, stat.l, stat.p
        ));
    }
    tournament_result
}
