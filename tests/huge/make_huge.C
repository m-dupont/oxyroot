#include "TFile.h"
#include "TTree.h"
#include "TBranch.h"


void make_huge()
{

    TFile *file = new TFile("huge.root", "RECREATE");
    TTree *tree = new TTree("tree", "");

    int int_array_25[25];

    tree->Branch("int_array_25", &int_array_25, "int_array_25[25]/I");

    for (int i = 0; i < 300000; i++) {
        for (int k = 0; k < 25; k++) {
            int_array_25[k] = i + k;
        }
        tree->Fill();
    }


    tree->Write();
    file->Close();

}
