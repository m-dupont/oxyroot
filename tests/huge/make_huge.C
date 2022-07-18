#include "TFile.h"
#include "TTree.h"
#include "TBranch.h"


void make_huge()
{

    TFile *file = new TFile("huge.root", "RECREATE");
    TTree *tree = new TTree("tree", "");

    int int_array_25[25];
    tree->Branch("int_array_25", &int_array_25, "int_array_25[25]/I");

    vector<int> int_vector;
    tree->Branch("int_vector", &int_vector);

    for (int i = 0; i < 300000; i++) {
        for (int k = 0; k < 25; k++) {
            int_array_25[k] = i + k;
        }
        if (i % 25 == 0) {
            //int_vector.clear();
            //int_vector.push_back(-1);
            //	  cout << "i = " << i  << " clear\n";
        }
        //	cout << "i = " << i  << "i%25 = " << i%25 << "\n";
        int_vector.clear();
        for (int k = 0; k < i % 25; ++k) {
            int_vector.push_back(i + k);
        }


        cout << "i = " << " len = " << int_vector.size() << endl;
        tree->Fill();
    }


    tree->Write();
    file->Close();

}
