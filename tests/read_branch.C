
void read_branch()
{
    TFile *f = new TFile("/tmp/rust/write/vector/i32/5.root");
    TTree *t = (TTree*)f->Get("mytree");
    vector<int32_t> *v = 0;
    t->SetBranchAddress("vector", &v);
    cout << ">>ENTRIES: " << t->GetEntries() << endl;
    for (int i = 0; i < t->GetEntries(); i++)
    {
        t->GetEntry(i);
        cout <<">>"<< v->size() << ";";
        for (int j = 0; j < v->size(); j++)
        {
            cout << v->at(j) << " ";
        }
        cout << endl;
    }
    f->Close();
}