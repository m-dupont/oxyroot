#include <iostream>

#include <TFile.h>
#include <TTree.h>

#include "data2Tree.hpp"

void t04_01_write_tree_points()
{
    TFile *ofile = TFile::Open("t04_01_write_tree_points.root", "recreate");
    if (!ofile) {
        std::cerr << " File not found or already exists" << std::endl;
        return;
    }
    TTree *myTree = new TTree("myTree", "");

    Point p;
    p.x = 0;
    p.y = 0;

    //obj_for_branch1.points.push_back(p);

    myTree->Branch("points", &p);

    for (int i = 0; i < 10; ++i) {
        p.x = i;
        p.y = i*i;

        myTree->Fill();
    }

    myTree->Print();

    myTree->Write();
    ofile->Close();

}

void t04_02_write_tree_points_prefix()
{
    TFile *ofile = TFile::Open("t04_02_write_tree_points_prefix.root", "recreate");
    if (!ofile) {
        std::cerr << " File not found or already exists" << std::endl;
        return;
    }
    TTree *myTree = new TTree("myTree", "");

    Point p;
    p.x = 0;
    p.y = 0;

    //obj_for_branch1.points.push_back(p);

    myTree->Branch("branch.", &p);

    for (int i = 0; i < 10; ++i) {
        p.x = i;
        p.y = i*i;

        myTree->Fill();
    }

    myTree->Print();

    myTree->Write();
    ofile->Close();

}

void t04_03_write_tree_pointsvector()
{
    TFile *ofile = TFile::Open("t04_03_write_tree_pointsvector.root", "recreate");
    if (!ofile) {
        std::cerr << " File not found or already exists" << std::endl;
        return;
    }
    TTree *myTree = new TTree("myTree", "");

    PointVector p;

    //obj_for_branch1.points.push_back(p);

    myTree->Branch("points", &p);

    for (int i = 0; i < 10; ++i) {
        p.x.clear();
        p.y.clear();
        for (int j = 0; j < i; ++j) {
            p.x.push_back(i);
            p.y.push_back(j);
        }


        myTree->Fill();
    }

    myTree->Print();

    myTree->Write();
    ofile->Close();

}

void t04_04_write_twopoints()
{
    TFile *ofile = TFile::Open("t04_04_write_twopoints.root", "recreate");
    if (!ofile) {
        std::cerr << " File not found or already exists" << std::endl;
        return;
    }
    TTree *myTree = new TTree("myTree", "");

    TwoPoints tp;

    //obj_for_branch1.points.push_back(p);

    myTree->Branch("points", &tp);

    for (int i = 0; i < 10; ++i) {
        tp.p1.x = i;
        tp.p2.y = i*i;

        myTree->Fill();
    }

    myTree->Print();

    myTree->Write();
    ofile->Close();

}

void t04_05_write_severalpoints()
{
    TFile *ofile = TFile::Open("t04_05_write_severalpoints.root", "recreate");
    if (!ofile) {
        std::cerr << " File not found or already exists" << std::endl;
        return;
    }
    TTree *myTree = new TTree("myTree", "");
    SeveralPoints tp;

    //obj_for_branch1.points.push_back(p);

    myTree->Branch("points", &tp);

    for (int i = 0; i < 10; ++i) {
        Point p;
        p.x = i;
        p.y = i*i;
        tp.points.push_back(p);

        myTree->Fill();
    }

    myTree->Print();

    myTree->Write();
    ofile->Close();

}

int main()
{
//    t04_01_write_tree_points();
//    t04_02_write_tree_points_prefix();
//    t04_03_write_tree_pointsvector();
//    t04_04_write_twopoints();
    t04_05_write_severalpoints();

   return 0;
}