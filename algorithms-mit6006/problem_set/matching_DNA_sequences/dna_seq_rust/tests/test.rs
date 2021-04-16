mod test {
    use dna_seq_rust::dnaseqlib::RollingHash;
    use dna_seq_rust::ExactSubMatchesIterator;
    use dna_seq_rust::Multidict;
    use dna_seq_rust::SubSequenceHashIterator;

    #[test]
    fn test_rolling_hash() {
        let mut rh1 = RollingHash::new(&"CTAGC".as_bytes());
        let rh2 = RollingHash::new(&"TAGCG".as_bytes());
        let rh3 = RollingHash::new(&"AGCGT".as_bytes());
        rh1.slide(b'C', b'G');
        assert_eq!(rh1.current_hash(), rh2.current_hash());
        rh1.slide(b'T', b'T');
        assert_eq!(rh1.current_hash(), rh3.current_hash());
    }

    #[test]
    fn test_multidict() {
        let mut foo: Multidict<char> = Multidict::new(Vec::new());
        foo.put(1, 'a');
        foo.put(2, 'b');
        foo.put(1, 'c');
        assert_eq!(foo.get(&1), vec!['a', 'c']);
        assert_eq!(foo.get(&2), vec!['b']);
        assert_eq!(foo.get(&3).len(), 0);
    }

    #[test]
    fn test_exact_sub_matches() {
        let foo = "yabcabcabcz";

        let mut bytes = foo.as_bytes().iter();
        let iterator = SubSequenceHashIterator::new(&mut bytes, 3);
        assert_eq!(iterator.count(), 9)
    }

    // This test case may break once you add the argument m (skipping).
    #[test]
    #[ignore]
    fn test_sub_sequence_hash_iterator() {
        let foo = "yabcabcabcz";
        let bar = "xxabcxxxx";

        let mut a_bytes = foo.as_bytes().iter();
        let mut b_bytes = bar.as_bytes().iter();

        let matches: Vec<(usize, usize)> =
            ExactSubMatchesIterator::new(&mut a_bytes, &mut b_bytes, 3, 1)
                .flatten()
                .collect();
        let correct = vec![(1, 2), (4, 2), (7, 2)];
        assert_eq!(matches.len(), correct.len());
        for x in correct {
            assert!(matches.contains(&x))
        }
    }
}
