// start of payload in a TFile (in bytes)
pub(crate) const kBEGIN: i64 = 100;

// kStartBigFile-1 is the largest position in a ROOT file before switching to
// the "big file" scheme (supporting files bigger than 4Gb) of ROOT.
pub(crate) const kStartBigFile: i64 = 2000000000;
