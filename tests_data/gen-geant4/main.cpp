#include <iostream>
#include <G4RunManager.hh>

#include "G4RootAnalysisManager.hh"
using namespace std;

int make_file_0(string dir_output) {
    G4RootAnalysisManager* analysisManager = G4RootAnalysisManager::Instance();
    analysisManager->OpenFile(dir_output + "/g4-ntuples.root");
    analysisManager->SetVerboseLevel(1);
    analysisManager->SetActivation(true);

    G4cout << "CreateNtuple = " << analysisManager->CreateNtuple("mytree-1000", "Table") << G4endl;
    G4cout << "CreateNtupleIColumn = " << analysisManager->CreateNtupleIColumn("i32");
    G4cout << "CreateNtupleIColumn = " << analysisManager->CreateNtupleIColumn("i64");
    G4cout << "CreateNtupleDColumn = " << analysisManager->CreateNtupleDColumn("f64");
    analysisManager->FinishNtuple(0);


    for (int i = 0; i < 1000; i++) {
        analysisManager->FillNtupleIColumn(0, 0, i);
        analysisManager->FillNtupleIColumn(0, 1, i*i);
        analysisManager->AddNtupleRow(0);
    }

    G4cout << "CreateNtuple = " << analysisManager->CreateNtuple("mytree-10000", "Table") << G4endl;
    G4cout << "CreateNtupleIColumn = " << analysisManager->CreateNtupleIColumn("i32");
    G4cout << "CreateNtupleIColumn = " << analysisManager->CreateNtupleIColumn("i64");
    G4cout << "CreateNtupleDColumn = " << analysisManager->CreateNtupleDColumn("f64");
    analysisManager->FinishNtuple(1);


    for (int i = 0; i < 10000; i++) {
        analysisManager->FillNtupleIColumn(1, 0, i);
        analysisManager->FillNtupleIColumn(1, 1, i*i);
        analysisManager->AddNtupleRow(1);
    }




    analysisManager->Write();
    analysisManager->CloseFile();

}



int main(int argc,char** argv) {

    if (argc <= 1) {
        std::cout << "Usage: " << argv[0] << " <dir output>" << std::endl;
        return 1;
    }

    std::string dir_output = argv[1];

    std::cout << "dir_output = " << dir_output << std::endl;
    make_file_0(dir_output);
//    make_file_1(dir_output);




//    G4RunManager * runManager = new G4RunManager;








    return 0;
}
