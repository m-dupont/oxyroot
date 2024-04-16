#include "TInterpreter.h"
#include "TFile.h"
#include "TTree.h"
#include "TBranch.h"
#include "TString.h"

#include <iostream>
#include <vector>
#include <set>
#include <map>

void tree_with_user_info_with_map()
{

     TFile *file = new TFile("tree_with_user_info_with_map.root", "RECREATE");
     TMap *m = new TMap();
     TObjString *key = new TObjString("key!");
     TObjString *val = new TObjString("value!");
     m->Add(key, val);
//     m->Write("map", TObject::kSingleKey, 0);

    TTree *tree = new TTree("tree", "");

//    tree->Branch("map", &m);

    std::string string;
    tree->Branch("string", &string);
    string.assign("0");

     tree->Fill();
    string.assign("string");
    tree->Fill();

    tree->GetUserInfo()->Add(m);
    TObjString *info = new TObjString("info!");
    tree->GetUserInfo()->Add(info);

    tree->Write();
    file->Close();
}
