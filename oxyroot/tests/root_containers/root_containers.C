#include "TInterpreter.h"
#include "TFile.h"
#include "TTree.h"
#include "TBranch.h"
#include "TString.h"

#include <iostream>
#include <vector>
#include <set>
#include <map>

void root_containers()
{

     TFile *file = new TFile("root_containers.root", "RECREATE");
     TMap *m = new TMap();
     TObjString *key = new TObjString("key!");
     TObjString *val = new TObjString("value!");
     m->Add(key, val);
     m->Write("map", TObject::kSingleKey, 0);

    TTree *tree = new TTree("tree", "");

    tree->Branch("map", &m);

    std::string string;
    tree->Branch("string", &string);

    TString tstring;
    tree->Branch("tstring", &tstring);


     tree->Fill();
    string.assign("string");
    tree->Fill();







    tree->Write();
    file->Close();
}
