
void read_branch()
{
    TFile *f = new TFile("/tmp/rust/write/i32/5.root");
    TTree *t = (TTree*)f->Get("mytree");
    int32_t n = 0;
    t->SetBranchAddress("i32", &n);
    cout << ">>ENTRIES: " << t->GetEntries() << endl;
    for (int i = 0; i < t->GetEntries(); i++)
    {
        t->GetEntry(i);
        cout <<">>n: "<< static_cast<int16_t>(n) << endl;
    }
    f->Close();
}