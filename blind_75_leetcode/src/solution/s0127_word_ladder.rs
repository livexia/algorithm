#![allow(dead_code)]
pub struct Solution {}

use std::collections::{HashMap, HashSet, VecDeque};
type Map = HashMap<String, usize>;
type Edges = Vec<Vec<usize>>;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // Solution::ladder_length_base_bfs(begin_word, end_word, word_list)
        Solution::ladder_length_bfs(begin_word, end_word, word_list)
    }

    fn ladder_length_bfs(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // indexing word
        let mut last = 0;
        let mut word_map: Map = HashMap::new();
        let mut edges: Edges = vec![];
        Solution::insert_edge(begin_word.clone(), &mut last, &mut word_map, &mut edges);
        for word in word_list {
            Solution::insert_edge(word, &mut last, &mut word_map, &mut edges);
        }
        if !word_map.contains_key(&end_word) {
            return 0;
        }
        let &begin_id = word_map.get(&begin_word).unwrap();
        let &end_id = word_map.get(&end_word).unwrap();

        let mut dis = vec![last + 1; last];
        dis[begin_id] = 2;

        let mut queue = VecDeque::new();
        queue.push_back(begin_id);
        while let Some(cur) = queue.pop_front() {
            if cur == end_id {
                return dis[end_id] as i32 / 2;
            }
            for &next in &edges[cur] {
                if dis[next] == last + 1 {
                    dis[next] = dis[cur] + 1;
                    queue.push_back(next);
                }
            }
        }
        0
    }

    fn insert_word(word: String, last: &mut usize, word_map: &mut Map, edges: &mut Edges) -> usize {
        match word_map.get(&word) {
            None => {
                word_map.insert(word, *last);
                edges.push(vec![]);
                *last += 1;
                *last - 1
            }
            Some(&i) => i,
        }
    }

    fn insert_edge(word: String, last: &mut usize, word_map: &mut Map, edges: &mut Edges) {
        let id1 = Solution::insert_word(word.clone(), last, word_map, edges);
        for i in 0..word.len() {
            let mut temp = String::new();
            temp.push_str(&word[0..i]);
            temp.push('*');
            temp.push_str(&word[i + 1..]);
            let id2 = Solution::insert_word(temp, last, word_map, edges);
            edges[id1].push(id2);
            edges[id2].push(id1);
        }
    }

    fn ladder_length_base_bfs(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_length = begin_word.len();
        let mut chars: Vec<HashSet<u8>> = vec![HashSet::new(); word_length];
        for word in &word_list {
            for (i, c) in word.bytes().enumerate() {
                chars[i].insert(c - 'a' as u8);
            }
        }
        let word_list: HashSet<u64> = word_list
            .into_iter()
            .map(|s| Solution::word_to_num(s))
            .collect();
        let end_word = Solution::word_to_num(end_word);
        let begin_word = Solution::word_to_num(begin_word);
        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut queue: VecDeque<(u64, i32)> = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((begin_word, 1));
        while let Some((cur, count)) = queue.pop_front() {
            if cur == end_word {
                return count;
            }
            if visited.insert(cur.clone()) {
                for i in 0..word_length {
                    for &c in &chars[i] {
                        let next = Solution::replace_at(cur, c, i, word_length);
                        if cur == next {
                            continue;
                        }
                        if !word_list.contains(&next) {
                            continue;
                        }
                        queue.push_back((next, count + 1));
                    }
                }
            }
        }
        0
    }

    fn word_to_vec(word: String) -> Vec<u8> {
        word.bytes().map(|c| c as u8 - 'a' as u8 + 1).collect()
    }

    fn vec_to_num(word: Vec<u8>) -> u64 {
        word.into_iter().fold(0, |sum, i| sum * 27 + i as u64)
    }

    fn word_to_num(word: String) -> u64 {
        word.bytes()
            .map(|c| c as u8 - 'a' as u8 + 1)
            .fold(0, |sum, i| sum * 27 + i as u64)
    }

    fn replace_at(a: u64, c: u8, index: usize, word_length: usize) -> u64 {
        let offset = 27u64.pow((word_length - 1 - index) as u32);
        a - ((a / offset) % 27) * offset + (c as u64 + 1) * offset
    }

    fn num_to_vec(num: u64) -> Vec<u8> {
        // 26 won't work 'aa' -> 00 -> 'a'
        // 27 will work 'aa' -> 11 -> 'aa'
        let mut num = num;
        let mut res = vec![];
        while num != 0 {
            res.push((num % 27) as u8);
            num /= 27;
        }
        res.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests_127 {
    use super::*;

    #[test]
    fn test_word_to_num() {
        assert_eq!(
            28,
            Solution::vec_to_num(Solution::word_to_vec("aa".to_string()))
        );
        assert_eq!(
            26,
            Solution::vec_to_num(Solution::word_to_vec("z".to_string()))
        );
        assert_eq!(vec![26], Solution::word_to_vec("z".to_string()));
        assert_eq!(
            56,
            Solution::vec_to_num(Solution::word_to_vec("bb".to_string()))
        );
        assert_eq!(
            Solution::word_to_vec("bb".to_string()),
            Solution::num_to_vec(56)
        );
        assert_eq!(
            Solution::word_to_vec("aa".to_string()),
            Solution::num_to_vec(28)
        );
        assert_eq!(
            Solution::word_to_vec("z".to_string()),
            Solution::num_to_vec(26)
        );
        assert_eq!(
            Solution::replace_at(28, 1, 0, 2),
            Solution::vec_to_num([2, 1].to_vec())
        );
    }

    fn ladder_length(begin_word: &str, end_word: &str, word_list: Vec<&str>) -> i32 {
        Solution::ladder_length(
            begin_word.to_string(),
            end_word.to_string(),
            word_list.into_iter().map(|s| s.to_string()).collect(),
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(
            ladder_length("hit", "cog", vec!["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );
        assert_eq!(
            ladder_length("hit", "cog", vec!["hot", "dot", "dog", "lot", "log"]),
            0
        );
        assert_eq!(
            ladder_length(
                "ymain",
                "oecij",
                vec![
                    "ymann", "yycrj", "oecij", "ymcnj", "yzcrj", "yycij", "xecij", "yecij",
                    "ymanj", "yzcnj", "ymain"
                ]
            ),
            10
        );
        assert_eq!(
            ladder_length(
                "raining",
                "cellini",
                vec![
                    "heaping", "conning", "nipping", "wadding", "pulling", "lunging", "figging",
                    "donning", "jamming", "coating", "foaling", "ousting", "dowsing", "busting",
                    "penning", "lapping", "yanking", "sapping", "tasking", "rigging", "ranking",
                    "larking", "farming", "dunging", "nutting", "gouging", "barfing", "fasting",
                    "belting", "boiling", "boating", "dipping", "kilning", "barking", "furling",
                    "calving", "veiling", "decking", "ricking", "salting", "lucking", "sending",
                    "taiping", "marking", "martina", "warping", "bulking", "seaming", "topping",
                    "larding", "jilting", "besting", "weeding", "nesting", "baiting", "jibbing",
                    "pelting", "bushing", "garbing", "banting", "keeping", "venting", "rapping",
                    "binning", "mulling", "smiting", "hatting", "tapping", "writing", "footing",
                    "carding", "ratting", "bagging", "sitting", "dousing", "pinking", "testing",
                    "passing", "gelling", "gassing", "ranging", "hefting", "vamping", "wetting",
                    "paining", "rolling", "sinking", "yakking", "shaking", "nabbing", "licking",
                    "sparing", "hamming", "celling", "halving", "matting", "landing", "kooking",
                    "pinning", "hagging", "narking", "soaping", "winding", "dealing", "earring",
                    "cunning", "moating", "skiting", "jutting", "fueling", "hooping", "guiling",
                    "mapping", "hailing", "gutting", "firming", "bunting", "mealing", "rending",
                    "jobbing", "pauling", "foiling", "peeking", "rollins", "lansing", "felling",
                    "whiting", "vealing", "resting", "saltine", "earning", "purging", "mullins",
                    "pausing", "colling", "banning", "wasting", "sealing", "gigging", "scaring",
                    "pocking", "massing", "curring", "storing", "dinging", "handing", "pitting",
                    "faining", "cupping", "staring", "riffing", "gowning", "hipping", "vanning",
                    "darting", "maiming", "damping", "deaning", "bellini", "kipling", "marting",
                    "hawking", "fending", "kicking", "beading", "curving", "wending", "yelling",
                    "foaming", "rifting", "surging", "gaining", "stoking", "panging", "winking",
                    "nursing", "oinking", "looking", "tolling", "bailing", "tanking", "hacking",
                    "warming", "cooping", "wanting", "rotting", "kinking", "bugging", "purling",
                    "wincing", "joining", "belling", "wilting", "tensing", "fellini", "wilding",
                    "binding", "bugling", "sagging", "nagging", "binging", "tatting", "cellini",
                    "silting", "belying", "ripping", "crating", "slaking", "killing", "hurting",
                    "running", "harming", "banding", "rinking", "staying", "touting", "hasting",
                    "melting", "nibbing", "talking", "ganging", "bonging", "rilling", "damning",
                    "pooling", "porting", "sinning", "collins", "barbing", "bunking", "smiling",
                    "hanging", "tending", "bulging", "ginning", "coiling", "lolling", "molting",
                    "letting", "mending", "hinging", "gunning", "melding", "dilling", "shaving",
                    "harping", "basting", "cobbing", "carting", "leading", "styling", "fowling",
                    "goading", "yowling", "zipping", "wagging", "gaoling", "panning", "valving",
                    "peeling", "titling", "sailing", "harding", "parring", "haloing", "quiting",
                    "punting", "reeling", "batting", "signing", "pegging", "bogging", "mashing",
                    "rankine", "seeding", "sassing", "wafting", "winging", "framing", "rooting",
                    "longing", "sabling", "bulbing", "whiling", "balking", "canting", "dashing",
                    "dueling", "renting", "booting", "whaling", "vatting", "veining", "fencing",
                    "yucking", "slaving", "welling", "sunning", "lulling", "purring", "dawning",
                    "sensing", "meaning", "wording", "hogging", "parsing", "falling", "yelping",
                    "dinning", "vetting", "hulling", "reading", "lapsing", "tooling", "hoaxing",
                    "roiling", "forming", "ramming", "gelding", "felting", "popping", "walling",
                    "costing", "welding", "washing", "filling", "lasting", "couping", "cabling",
                    "getting", "winning", "carping", "martins", "bilking", "burning", "jelling",
                    "sicking", "tinting", "ceiling", "totting", "balding", "kenning", "tinging",
                    "hugging", "westing", "burring", "pasting", "pecking", "parking", "slaying",
                    "pigging", "heating", "manning", "bucking", "bussing", "gagging", "goaling",
                    "rowling", "netting", "funking", "pitying", "jarring", "tasting", "putting",
                    "beating", "funding", "mauling", "balling", "molding", "shining", "perkins",
                    "dialing", "panting", "looping", "welting", "relying", "dulling", "dumping",
                    "tanning", "warring", "gatling", "staging", "finding", "farting", "petting",
                    "picking", "swaying", "toiling", "jambing", "bawling", "minting", "wedding",
                    "hulking", "keeling", "nanking", "railing", "heading", "cutting", "gosling",
                    "vesting", "sighing", "mucking", "copping", "polling", "raising", "fooling",
                    "hooting", "titting", "calming", "seating", "rifling", "soiling", "dubbing",
                    "jesting", "posting", "sacking", "corking", "yipping", "lathing", "bopping",
                    "setting", "coaxing", "poshing", "fawning", "heeling", "warning", "napping",
                    "vending", "mooting", "hurling", "supping", "nanjing", "pipping", "tagging",
                    "mopping", "souping", "palming", "gulling", "kirking", "gilding", "docking",
                    "wefting", "dusting", "sharing", "darling", "bowling", "lauding", "bidding",
                    "hopping", "honking", "hunting", "pepping", "busying", "damming", "patting",
                    "hitting", "gusting", "jigging", "gabbing", "hosting", "sidling", "telling",
                    "rusting", "daubing", "reining", "memling", "healing", "gashing", "betting",
                    "lilting", "hashing", "salving", "firring", "gabling", "ducking", "waiving",
                    "skating", "worming", "waiting", "burying", "booking", "corning", "suiting",
                    "hooking", "gonging", "listing", "hulaing", "sulking", "digging", "fouling",
                    "zincing", "cocking", "packing", "scaling", "pooping", "zinging", "banging",
                    "bolling", "punning", "palling", "sipping", "bunging", "minding", "choking",
                    "yapping", "nicking", "warding", "gorging", "canning", "culling", "lending",
                    "spaying", "lashing", "pupping", "fanning", "banking", "pinging", "roaming",
                    "sopping", "fonding", "searing", "fucking", "rooking", "tooting", "raining",
                    "billing", "pulsing", "curbing", "cashing", "calking", "harking", "tarring",
                    "tacking", "whining", "tarting", "pauline", "rasping", "howling", "helling",
                    "curling", "pucking", "hauling", "coaling", "lopping", "mailing", "wailing",
                    "lugging", "ticking", "staving", "snaking", "selling", "masking", "jabbing",
                    "mewling", "heaving", "soaring", "fagging", "cording", "begging", "ridging",
                    "jetting", "backing", "dotting", "lacking", "parting", "jotting", "dunning",
                    "tinning", "stiling", "stating", "zapping", "hearing", "fitting", "barging",
                    "galling", "wigging", "feeding", "tenting", "looting", "cabbing", "cursing",
                    "dunking", "dabbing", "ragging", "bedding", "witting", "pouting", "burping",
                    "slating", "tamping", "basking", "failing", "papping", "narcing", "lancing",
                    "furlong", "tabling", "dolling", "tailing", "pawning", "collies", "lamming",
                    "coifing", "bolting", "sucking", "rafting", "morning", "ranting", "tabbing",
                    "rinding", "bandung", "bashing", "bending", "ducting", "casting", "camping",
                    "flaming", "hinting", "sanding", "carving", "lagging", "helping", "keening",
                    "jolting", "temping", "junking", "manging", "dimming", "ringing", "tipping",
                    "spiking", "malling", "pursing", "soaking", "willing", "fulling", "causing",
                    "jacking", "furring", "singing", "halting", "tucking", "ruining", "denting",
                    "calling", "barring", "fopping", "yawning", "tilling", "nilling", "downing",
                    "cooling", "martini", "budging", "lapwing", "mincing", "rinsing", "cowling",
                    "marring", "coining", "sibling", "potting", "tauting", "bulling", "lurking",
                    "sorting", "poohing", "bathing", "spicing", "nailing", "spiting", "racking",
                    "lusting", "rutting", "gulping", "tilting", "pairing", "peaking", "capping",
                    "gobbing", "finking"
                ]
            ),
            7
        );
    }
}
