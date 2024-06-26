#include "TInterpreter.h"
#include "TFile.h"
#include "TTree.h"
#include "TBranch.h"
#include "TString.h"

#include <iostream>
#include <vector>
#include <set>
#include <map>

void stl_containers()
{
    TFile *file = new TFile("stl_containers.root", "RECREATE");
    TTree *tree = new TTree("tree", "");

    std::string string;
    tree->Branch("string", &string);

    TString tstring;
    tree->Branch("tstring", &tstring);

    std::vector <int32_t> vector_int32;
    tree->Branch("vector_int32", &vector_int32);

    std::vector <std::string> vector_string;
    tree->Branch("vector_string", &vector_string);

    std::vector <TString> vector_tstring;
    tree->Branch("vector_tstring", &vector_tstring);

    std::vector <std::vector<int32_t>> vector_vector_int32;
    gInterpreter->GenerateDictionary("vector<vector<int> >", "vector");
    tree->Branch("vector_vector_int32", &vector_vector_int32);

    std::vector <std::vector<std::string>> vector_vector_string;
    gInterpreter->GenerateDictionary("vector<vector<string> >", "vector;string");
    tree->Branch("vector_vector_string", &vector_vector_string);

    std::vector <std::vector<TString>> vector_vector_tstring;
    gInterpreter->GenerateDictionary("vector<vector<TString> >", "vector");
    tree->Branch("vector_vector_tstring", &vector_vector_tstring);

    std::vector <std::set<int32_t>> vector_set_int32;
    gInterpreter->GenerateDictionary("vector<set<int> >", "vector;set");
    tree->Branch("vector_set_int32", &vector_set_int32);

    std::vector <std::set<std::string>> vector_set_string;
    gInterpreter->GenerateDictionary("vector<set<string> >", "vector;set;string");
    tree->Branch("vector_set_string", &vector_set_string);

    std::set <int32_t> set_int32;
    tree->Branch("set_int32", &set_int32);

    std::set <std::string> set_string;
    tree->Branch("set_string", &set_string);

    std::map <int32_t, int16_t> map_int32_int16;
    gInterpreter->GenerateDictionary("map<int,short>", "map");
    tree->Branch("map_int32_int16", &map_int32_int16, 32000, 0);

    std::map <int32_t, std::vector<int16_t>> map_int32_vector_int16;
    gInterpreter->GenerateDictionary("map<int,vector<short> >", "map;vector");
    tree->Branch("map_int32_vector_int16", &map_int32_vector_int16, 32000, 0);

    std::map <int32_t, std::vector<std::string>> map_int32_vector_string;
    gInterpreter->GenerateDictionary("map<int,vector<string> >", "map;vector;string");
    tree->Branch("map_int32_vector_string", &map_int32_vector_string, 32000, 0);

    std::map <int32_t, std::set<int16_t>> map_int32_set_int16;
    gInterpreter->GenerateDictionary("map<int,set<short> >", "map;set");
    tree->Branch("map_int32_set_int16", &map_int32_set_int16, 32000, 0);

    std::map <int32_t, std::set<std::string>> map_int32_set_string;
    gInterpreter->GenerateDictionary("map<int,set<string> >", "map;set;string");
    tree->Branch("map_int32_set_string", &map_int32_set_string, 32000, 0);

    std::map <std::string, int16_t> map_string_int16;
    gInterpreter->GenerateDictionary("map<string,short>", "map;string");
    tree->Branch("map_string_int16", &map_string_int16, 32000, 0);

    std::map <std::string, std::vector<int16_t>> map_string_vector_int16;
    gInterpreter->GenerateDictionary("map<string,vector<short> >", "map;vector;string");
    tree->Branch("map_string_vector_int16", &map_string_vector_int16, 32000, 0);

    std::map <std::string, std::vector<std::string>> map_string_vector_string;
    gInterpreter->GenerateDictionary("map<string,vector<string> >", "map;vector;string");
    tree->Branch("map_string_vector_string", &map_string_vector_string, 32000, 0);

    std::map <std::string, std::set<int16_t>> map_string_set_int16;
    gInterpreter->GenerateDictionary("map<string,set<short> >", "map;set;string");
    tree->Branch("map_string_set_int16", &map_string_set_int16, 32000, 0);

    std::map <std::string, std::set<std::string>> map_string_set_string;
    gInterpreter->GenerateDictionary("map<string,set<string> >", "map;set;string");
    tree->Branch("map_string_set_string", &map_string_set_string, 32000, 0);

    std::map < int32_t, std::vector < std::vector < int16_t > > > map_int32_vector_vector_int16;
    gInterpreter->GenerateDictionary("map<int,vector<vector<short> > >", "map;vector");
    tree->Branch("map_int32_vector_vector_int16", &map_int32_vector_vector_int16, 32000, 0);

    std::map < int32_t, std::vector < std::set < int16_t > > > map_int32_vector_set_int16;
    gInterpreter->GenerateDictionary("map<int,vector<set<short> > >", "map;vector;set");
    tree->Branch("map_int32_vector_set_int16", &map_int32_vector_set_int16, 32000, 0);

    std::map <std::string, std::string> map_string_string;
    gInterpreter->GenerateDictionary("map<string,string>", "map;string");
    tree->Branch("map_string_string", &map_string_string, 32000, 0);

    std::map <std::string, TString> map_string_tstring;
    gInterpreter->GenerateDictionary("map<string,TString>", "map;string");
    tree->Branch("map_string_tstring", &map_string_tstring, 32000, 0);

    string.clear();
    string.assign("one");
    tstring.Clear();
    tstring.Append("one");
    vector_int32.clear();
    vector_int32.push_back(1);
    vector_string.clear();
    vector_string.push_back("one");
    vector_tstring.clear();
    vector_tstring.push_back("one");
    vector_vector_int32.clear();
    vector_vector_int32.push_back(std::vector < int32_t > {1});
    vector_vector_string.clear();
    vector_vector_string.push_back(std::vector < std::string > {"one"});
    vector_vector_tstring.clear();
    vector_vector_tstring.push_back(std::vector < TString > {"one"});
    vector_set_int32.clear();
    vector_set_int32.push_back(std::set < int32_t > {1});
    vector_set_string.clear();
    vector_set_string.push_back(std::set < std::string > {"one"});
    set_int32.clear();
    set_int32.insert(1);
    set_string.clear();
    set_string.insert("one");
    map_int32_int16.clear();
    map_int32_int16[1] = 1;
    map_int32_vector_int16.clear();
    map_int32_vector_int16[1] = std::vector<int16_t>({1});
    map_int32_vector_string.clear();
    map_int32_vector_string[1] = std::vector<std::string>({"one"});
    map_int32_set_int16.clear();
    map_int32_set_int16[1] = std::set<int16_t>({1});
    map_int32_set_string.clear();
    map_int32_set_string[1] = std::set<std::string>({"one"});
    map_string_int16.clear();
    map_string_int16["one"] = 1;
    map_string_vector_int16.clear();
    map_string_vector_int16["one"] = std::vector<int16_t>({1});
    map_string_vector_string.clear();
    map_string_vector_string["one"] = std::vector<std::string>({"one"});
    map_string_set_int16.clear();
    map_string_set_int16["one"] = std::set<int16_t>({1});
    map_string_set_string.clear();
    map_string_set_string["one"] = std::set<std::string>({"one"});
    map_int32_vector_vector_int16.clear();
    map_int32_vector_vector_int16[1] = std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}};
    map_int32_vector_set_int16.clear();
    map_int32_vector_set_int16[1] = std::vector < std::set < int16_t >> {std::set < int16_t > {1}};
    map_string_string.clear();
    map_string_string["one"] = "ONE";
    map_string_tstring.clear();
    map_string_tstring["one"] = "ONE";

    tree->Fill();

    string.clear();
    string.assign("two");
    tstring.Clear();
    tstring.Append("two");
    vector_int32.clear();
    vector_int32.push_back(1);
    vector_int32.push_back(2);
    vector_string.clear();
    vector_string.push_back("one");
    vector_string.push_back("two");
    vector_tstring.clear();
    vector_tstring.push_back("one");
    vector_tstring.push_back("two");
    vector_vector_int32.clear();
    vector_vector_int32.push_back(std::vector < int32_t > {1});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2});
    vector_vector_string.clear();
    vector_vector_string.push_back(std::vector < std::string > {"one"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two"});
    vector_vector_tstring.clear();
    vector_vector_tstring.push_back(std::vector < TString > {"one"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two"});
    vector_set_int32.clear();
    vector_set_int32.push_back(std::set < int32_t > {1});
    vector_set_int32.push_back(std::set < int32_t > {1, 2});
    vector_set_string.clear();
    vector_set_string.push_back(std::set < std::string > {"one"});
    vector_set_string.push_back(std::set < std::string > {"one", "two"});
    set_int32.clear();
    set_int32.insert(1);
    set_int32.insert(2);
    set_string.clear();
    set_string.insert("one");
    set_string.insert("two");
    map_int32_int16.clear();
    map_int32_int16[1] = 1;
    map_int32_int16[2] = 2;
    map_int32_vector_int16.clear();
    map_int32_vector_int16[1] = std::vector<int16_t>({1});
    map_int32_vector_int16[2] = std::vector<int16_t>({1, 2});
    map_int32_vector_string.clear();
    map_int32_vector_string[1] = std::vector<std::string>({"one"});
    map_int32_vector_string[2] = std::vector<std::string>({"one", "two"});
    map_int32_set_int16.clear();
    map_int32_set_int16[1] = std::set<int16_t>({1});
    map_int32_set_int16[2] = std::set<int16_t>({1, 2});
    map_int32_set_string.clear();
    map_int32_set_string[1] = std::set<std::string>({"one"});
    map_int32_set_string[2] = std::set<std::string>({"one", "two"});
    map_string_int16.clear();
    map_string_int16["one"] = 1;
    map_string_int16["two"] = 2;
    map_string_vector_int16.clear();
    map_string_vector_int16["one"] = std::vector<int16_t>({1});
    map_string_vector_int16["two"] = std::vector<int16_t>({1, 2});
    map_string_vector_string.clear();
    map_string_vector_string["one"] = std::vector<std::string>({"one"});
    map_string_vector_string["two"] = std::vector<std::string>({"one", "two"});
    map_string_set_int16.clear();
    map_string_set_int16["one"] = std::set<int16_t>({1});
    map_string_set_int16["two"] = std::set<int16_t>({1, 2});
    map_string_set_string.clear();
    map_string_set_string["one"] = std::set<std::string>({"one"});
    map_string_set_string["two"] = std::set<std::string>({"one", "two"});
    map_int32_vector_vector_int16.clear();
    map_int32_vector_vector_int16[1] = std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}};
    map_int32_vector_vector_int16[2] =
            std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}};
    map_int32_vector_set_int16.clear();
    map_int32_vector_set_int16[1] = std::vector < std::set < int16_t >> {std::set < int16_t > {1}};
    map_int32_vector_set_int16[2] =
            std::vector < std::set < int16_t >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}};
    map_string_string.clear();
    map_string_string["one"] = "ONE";
    map_string_string["two"] = "TWO";
    map_string_tstring.clear();
    map_string_tstring["one"] = "ONE";
    map_string_tstring["two"] = "TWO";

    tree->Fill();

    string.clear();
    string.assign("three");
    tstring.Clear();
    tstring.Append("three");
    vector_int32.clear();
    vector_int32.push_back(1);
    vector_int32.push_back(2);
    vector_int32.push_back(3);
    vector_string.clear();
    vector_string.push_back("one");
    vector_string.push_back("two");
    vector_string.push_back("three");
    vector_tstring.clear();
    vector_tstring.push_back("one");
    vector_tstring.push_back("two");
    vector_tstring.push_back("three");
    vector_vector_int32.clear();
    vector_vector_int32.push_back(std::vector < int32_t > {1});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2, 3});
    vector_vector_string.clear();
    vector_vector_string.push_back(std::vector < std::string > {"one"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two", "three"});
    vector_vector_tstring.clear();
    vector_vector_tstring.push_back(std::vector < TString > {"one"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two", "three"});
    vector_set_int32.clear();
    vector_set_int32.push_back(std::set < int32_t > {1});
    vector_set_int32.push_back(std::set < int32_t > {1, 2});
    vector_set_int32.push_back(std::set < int32_t > {1, 2, 3});
    vector_set_string.clear();
    vector_set_string.push_back(std::set < std::string > {"one"});
    vector_set_string.push_back(std::set < std::string > {"one", "two"});
    vector_set_string.push_back(std::set < std::string > {"one", "two", "three"});
    set_int32.clear();
    set_int32.insert(1);
    set_int32.insert(2);
    set_int32.insert(3);
    set_string.clear();
    set_string.insert("one");
    set_string.insert("two");
    set_string.insert("three");
    map_int32_int16.clear();
    map_int32_int16[1] = 1;
    map_int32_int16[2] = 2;
    map_int32_int16[3] = 3;
    map_int32_vector_int16.clear();
    map_int32_vector_int16[1] = std::vector<int16_t>({1});
    map_int32_vector_int16[2] = std::vector<int16_t>({1, 2});
    map_int32_vector_int16[3] = std::vector<int16_t>({1, 2, 3});
    map_int32_vector_string.clear();
    map_int32_vector_string[1] = std::vector<std::string>({"one"});
    map_int32_vector_string[2] = std::vector<std::string>({"one", "two"});
    map_int32_vector_string[3] = std::vector<std::string>({"one", "two", "three"});
    map_int32_set_int16.clear();
    map_int32_set_int16[1] = std::set<int16_t>({1});
    map_int32_set_int16[2] = std::set<int16_t>({1, 2});
    map_int32_set_int16[3] = std::set<int16_t>({1, 2, 3});
    map_int32_set_string.clear();
    map_int32_set_string[1] = std::set<std::string>({"one"});
    map_int32_set_string[2] = std::set<std::string>({"one", "two"});
    map_int32_set_string[3] = std::set<std::string>({"one", "two", "three"});
    map_string_int16.clear();
    map_string_int16["one"] = 1;
    map_string_int16["two"] = 2;
    map_string_int16["three"] = 3;
    map_string_vector_int16.clear();
    map_string_vector_int16["one"] = std::vector<int16_t>({1});
    map_string_vector_int16["two"] = std::vector<int16_t>({1, 2});
    map_string_vector_int16["three"] = std::vector<int16_t>({1, 2, 3});
    map_string_vector_string.clear();
    map_string_vector_string["one"] = std::vector<std::string>({"one"});
    map_string_vector_string["two"] = std::vector<std::string>({"one", "two"});
    map_string_vector_string["three"] = std::vector<std::string>({"one", "two", "three"});
    map_string_set_int16.clear();
    map_string_set_int16["one"] = std::set<int16_t>({1});
    map_string_set_int16["two"] = std::set<int16_t>({1, 2});
    map_string_set_int16["three"] = std::set<int16_t>({1, 2, 3});
    map_string_set_string.clear();
    map_string_set_string["one"] = std::set<std::string>({"one"});
    map_string_set_string["two"] = std::set<std::string>({"one", "two"});
    map_string_set_string["three"] = std::set<std::string>({"one", "two", "three"});
    map_int32_vector_vector_int16.clear();
    map_int32_vector_vector_int16[1] = std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}};
    map_int32_vector_vector_int16[2] =
            std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}};
    map_int32_vector_vector_int16[3] = std::vector < std::vector < int16_t
            >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}, std::vector < int16_t > {1, 2, 3}};
    map_int32_vector_set_int16.clear();
    map_int32_vector_set_int16[1] = std::vector < std::set < int16_t >> {std::set < int16_t > {1}};
    map_int32_vector_set_int16[2] =
            std::vector < std::set < int16_t >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}};
    map_int32_vector_set_int16[3] = std::vector < std::set < int16_t
            >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}, std::set < int16_t > {1, 2, 3}};
    map_string_string.clear();
    map_string_string["one"] = "ONE";
    map_string_string["two"] = "TWO";
    map_string_string["three"] = "THREE";
    map_string_tstring.clear();
    map_string_tstring["one"] = "ONE";
    map_string_tstring["two"] = "TWO";
    map_string_tstring["three"] = "THREE";

    tree->Fill();

    string.clear();
    string.assign("four");
    tstring.Clear();
    tstring.Append("four");
    vector_int32.clear();
    vector_int32.push_back(1);
    vector_int32.push_back(2);
    vector_int32.push_back(3);
    vector_int32.push_back(4);
    vector_string.clear();
    vector_string.push_back("one");
    vector_string.push_back("two");
    vector_string.push_back("three");
    vector_string.push_back("four");
    vector_tstring.clear();
    vector_tstring.push_back("one");
    vector_tstring.push_back("two");
    vector_tstring.push_back("three");
    vector_tstring.push_back("four");
    vector_vector_int32.clear();
    vector_vector_int32.push_back(std::vector < int32_t > {1});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2, 3});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2, 3, 4});
    vector_vector_string.clear();
    vector_vector_string.push_back(std::vector < std::string > {"one"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two", "three"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two", "three", "four"});
    vector_vector_tstring.clear();
    vector_vector_tstring.push_back(std::vector < TString > {"one"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two", "three"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two", "three", "four"});
    vector_set_int32.clear();
    vector_set_int32.push_back(std::set < int32_t > {1});
    vector_set_int32.push_back(std::set < int32_t > {1, 2});
    vector_set_int32.push_back(std::set < int32_t > {1, 2, 3});
    vector_set_int32.push_back(std::set < int32_t > {1, 2, 3, 4});
    vector_set_string.clear();
    vector_set_string.push_back(std::set < std::string > {"one"});
    vector_set_string.push_back(std::set < std::string > {"one", "two"});
    vector_set_string.push_back(std::set < std::string > {"one", "two", "three"});
    vector_set_string.push_back(std::set < std::string > {"one", "two", "three", "four"});
    set_int32.clear();
    set_int32.insert(1);
    set_int32.insert(2);
    set_int32.insert(3);
    set_int32.insert(4);
    set_string.clear();
    set_string.insert("one");
    set_string.insert("two");
    set_string.insert("three");
    set_string.insert("four");
    map_int32_int16.clear();
    map_int32_int16[1] = 1;
    map_int32_int16[2] = 2;
    map_int32_int16[3] = 3;
    map_int32_int16[4] = 4;
    map_int32_vector_int16.clear();
    map_int32_vector_int16[1] = std::vector<int16_t>({1});
    map_int32_vector_int16[2] = std::vector<int16_t>({1, 2});
    map_int32_vector_int16[3] = std::vector<int16_t>({1, 2, 3});
    map_int32_vector_int16[4] = std::vector<int16_t>({1, 2, 3, 4});
    map_int32_vector_string.clear();
    map_int32_vector_string[1] = std::vector<std::string>({"one"});
    map_int32_vector_string[2] = std::vector<std::string>({"one", "two"});
    map_int32_vector_string[3] = std::vector<std::string>({"one", "two", "three"});
    map_int32_vector_string[4] = std::vector<std::string>({"one", "two", "three", "four"});
    map_int32_set_int16.clear();
    map_int32_set_int16[1] = std::set<int16_t>({1});
    map_int32_set_int16[2] = std::set<int16_t>({1, 2});
    map_int32_set_int16[3] = std::set<int16_t>({1, 2, 3});
    map_int32_set_int16[4] = std::set<int16_t>({1, 2, 3, 4});
    map_int32_set_string.clear();
    map_int32_set_string[1] = std::set<std::string>({"one"});
    map_int32_set_string[2] = std::set<std::string>({"one", "two"});
    map_int32_set_string[3] = std::set<std::string>({"one", "two", "three"});
    map_int32_set_string[4] = std::set<std::string>({"one", "two", "three", "four"});
    map_string_int16.clear();
    map_string_int16["one"] = 1;
    map_string_int16["two"] = 2;
    map_string_int16["three"] = 3;
    map_string_int16["four"] = 4;
    map_string_vector_int16.clear();
    map_string_vector_int16["one"] = std::vector<int16_t>({1});
    map_string_vector_int16["two"] = std::vector<int16_t>({1, 2});
    map_string_vector_int16["three"] = std::vector<int16_t>({1, 2, 3});
    map_string_vector_int16["four"] = std::vector<int16_t>({1, 2, 3, 4});
    map_string_vector_string.clear();
    map_string_vector_string["one"] = std::vector<std::string>({"one"});
    map_string_vector_string["two"] = std::vector<std::string>({"one", "two"});
    map_string_vector_string["three"] = std::vector<std::string>({"one", "two", "three"});
    map_string_vector_string["four"] = std::vector<std::string>({"one", "two", "three", "four"});
    map_string_set_int16.clear();
    map_string_set_int16["one"] = std::set<int16_t>({1});
    map_string_set_int16["two"] = std::set<int16_t>({1, 2});
    map_string_set_int16["three"] = std::set<int16_t>({1, 2, 3});
    map_string_set_int16["four"] = std::set<int16_t>({1, 2, 3, 4});
    map_string_set_string.clear();
    map_string_set_string["one"] = std::set<std::string>({"one"});
    map_string_set_string["two"] = std::set<std::string>({"one", "two"});
    map_string_set_string["three"] = std::set<std::string>({"one", "two", "three"});
    map_string_set_string["four"] = std::set<std::string>({"one", "two", "three", "four"});
    map_int32_vector_vector_int16.clear();
    map_int32_vector_vector_int16[1] = std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}};
    map_int32_vector_vector_int16[2] =
            std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}};
    map_int32_vector_vector_int16[3] = std::vector < std::vector < int16_t
            >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}, std::vector < int16_t > {1, 2, 3}};
    map_int32_vector_vector_int16[4] = std::vector < std::vector < int16_t
            >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}, std::vector < int16_t > {1, 2, 3},
                std::vector < int16_t > {1, 2, 3, 4}};
    map_int32_vector_set_int16.clear();
    map_int32_vector_set_int16[1] = std::vector < std::set < int16_t >> {std::set < int16_t > {1}};
    map_int32_vector_set_int16[2] =
            std::vector < std::set < int16_t >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}};
    map_int32_vector_set_int16[3] = std::vector < std::set < int16_t
            >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}, std::set < int16_t > {1, 2, 3}};
    map_int32_vector_set_int16[4] = std::vector < std::set < int16_t
            >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}, std::set < int16_t > {1, 2, 3},
                std::set < int16_t > {1, 2, 3, 4}};
    map_string_string.clear();
    map_string_string["one"] = "ONE";
    map_string_string["two"] = "TWO";
    map_string_string["three"] = "THREE";
    map_string_string["four"] = "FOUR";
    map_string_tstring.clear();
    map_string_tstring["one"] = "ONE";
    map_string_tstring["two"] = "TWO";
    map_string_tstring["three"] = "THREE";
    map_string_tstring["four"] = "FOUR";

    tree->Fill();

    string.clear();
    string.assign("five");
    tstring.Clear();
    tstring.Append("five");
    vector_int32.clear();
    vector_int32.push_back(1);
    vector_int32.push_back(2);
    vector_int32.push_back(3);
    vector_int32.push_back(4);
    vector_int32.push_back(5);
    vector_string.clear();
    vector_string.push_back("one");
    vector_string.push_back("two");
    vector_string.push_back("three");
    vector_string.push_back("four");
    vector_string.push_back("five");
    vector_tstring.clear();
    vector_tstring.push_back("one");
    vector_tstring.push_back("two");
    vector_tstring.push_back("three");
    vector_tstring.push_back("four");
    vector_tstring.push_back("five");
    vector_vector_int32.clear();
    vector_vector_int32.push_back(std::vector < int32_t > {1});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2, 3});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2, 3, 4});
    vector_vector_int32.push_back(std::vector < int32_t > {1, 2, 3, 4, 5});
    vector_vector_string.clear();
    vector_vector_string.push_back(std::vector < std::string > {"one"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two", "three"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two", "three", "four"});
    vector_vector_string.push_back(std::vector < std::string > {"one", "two", "three", "four", "five"});
    vector_vector_tstring.clear();
    vector_vector_tstring.push_back(std::vector < TString > {"one"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two", "three"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two", "three", "four"});
    vector_vector_tstring.push_back(std::vector < TString > {"one", "two", "three", "four", "five"});
    vector_set_int32.clear();
    vector_set_int32.push_back(std::set < int32_t > {1});
    vector_set_int32.push_back(std::set < int32_t > {1, 2});
    vector_set_int32.push_back(std::set < int32_t > {1, 2, 3});
    vector_set_int32.push_back(std::set < int32_t > {1, 2, 3, 4});
    vector_set_int32.push_back(std::set < int32_t > {1, 2, 3, 4, 5});
    vector_set_string.clear();
    vector_set_string.push_back(std::set < std::string > {"one"});
    vector_set_string.push_back(std::set < std::string > {"one", "two"});
    vector_set_string.push_back(std::set < std::string > {"one", "two", "three"});
    vector_set_string.push_back(std::set < std::string > {"one", "two", "three", "four"});
    vector_set_string.push_back(std::set < std::string > {"one", "two", "three", "four", "five"});
    set_int32.clear();
    set_int32.insert(1);
    set_int32.insert(2);
    set_int32.insert(3);
    set_int32.insert(4);
    set_int32.insert(5);
    set_string.clear();
    set_string.insert("one");
    set_string.insert("two");
    set_string.insert("three");
    set_string.insert("four");
    set_string.insert("five");
    map_int32_int16.clear();
    map_int32_int16[1] = 1;
    map_int32_int16[2] = 2;
    map_int32_int16[3] = 3;
    map_int32_int16[4] = 4;
    map_int32_int16[5] = 5;
    map_int32_vector_int16.clear();
    map_int32_vector_int16[1] = std::vector<int16_t>({1});
    map_int32_vector_int16[2] = std::vector<int16_t>({1, 2});
    map_int32_vector_int16[3] = std::vector<int16_t>({1, 2, 3});
    map_int32_vector_int16[4] = std::vector<int16_t>({1, 2, 3, 4});
    map_int32_vector_int16[5] = std::vector<int16_t>({1, 2, 3, 4, 5});
    map_int32_vector_string.clear();
    map_int32_vector_string[1] = std::vector<std::string>({"one"});
    map_int32_vector_string[2] = std::vector<std::string>({"one", "two"});
    map_int32_vector_string[3] = std::vector<std::string>({"one", "two", "three"});
    map_int32_vector_string[4] = std::vector<std::string>({"one", "two", "three", "four"});
    map_int32_vector_string[5] = std::vector<std::string>({"one", "two", "three", "four", "five"});
    map_int32_set_int16.clear();
    map_int32_set_int16[1] = std::set<int16_t>({1});
    map_int32_set_int16[2] = std::set<int16_t>({1, 2});
    map_int32_set_int16[3] = std::set<int16_t>({1, 2, 3});
    map_int32_set_int16[4] = std::set<int16_t>({1, 2, 3, 4});
    map_int32_set_int16[5] = std::set<int16_t>({1, 2, 3, 4, 5});
    map_int32_set_string.clear();
    map_int32_set_string[1] = std::set<std::string>({"one"});
    map_int32_set_string[2] = std::set<std::string>({"one", "two"});
    map_int32_set_string[3] = std::set<std::string>({"one", "two", "three"});
    map_int32_set_string[4] = std::set<std::string>({"one", "two", "three", "four"});
    map_int32_set_string[5] = std::set<std::string>({"one", "two", "three", "four", "five"});
    map_string_int16.clear();
    map_string_int16["one"] = 1;
    map_string_int16["two"] = 2;
    map_string_int16["three"] = 3;
    map_string_int16["four"] = 4;
    map_string_int16["five"] = 5;
    map_string_vector_int16.clear();
    map_string_vector_int16["one"] = std::vector<int16_t>({1});
    map_string_vector_int16["two"] = std::vector<int16_t>({1, 2});
    map_string_vector_int16["three"] = std::vector<int16_t>({1, 2, 3});
    map_string_vector_int16["four"] = std::vector<int16_t>({1, 2, 3, 4});
    map_string_vector_int16["five"] = std::vector<int16_t>({1, 2, 3, 4, 5});
    map_string_vector_string.clear();
    map_string_vector_string["one"] = std::vector<std::string>({"one"});
    map_string_vector_string["two"] = std::vector<std::string>({"one", "two"});
    map_string_vector_string["three"] = std::vector<std::string>({"one", "two", "three"});
    map_string_vector_string["four"] = std::vector<std::string>({"one", "two", "three", "four"});
    map_string_vector_string["five"] = std::vector<std::string>({"one", "two", "three", "four", "five"});
    map_string_set_int16.clear();
    map_string_set_int16["one"] = std::set<int16_t>({1});
    map_string_set_int16["two"] = std::set<int16_t>({1, 2});
    map_string_set_int16["three"] = std::set<int16_t>({1, 2, 3});
    map_string_set_int16["four"] = std::set<int16_t>({1, 2, 3, 4});
    map_string_set_int16["five"] = std::set<int16_t>({1, 2, 3, 4, 5});
    map_string_set_string.clear();
    map_string_set_string["one"] = std::set<std::string>({"one"});
    map_string_set_string["two"] = std::set<std::string>({"one", "two"});
    map_string_set_string["three"] = std::set<std::string>({"one", "two", "three"});
    map_string_set_string["four"] = std::set<std::string>({"one", "two", "three", "four"});
    map_string_set_string["five"] = std::set<std::string>({"one", "two", "three", "four", "five"});
    map_int32_vector_vector_int16.clear();
    map_int32_vector_vector_int16[1] = std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}};
    map_int32_vector_vector_int16[2] =
            std::vector < std::vector < int16_t >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}};
    map_int32_vector_vector_int16[3] = std::vector < std::vector < int16_t
            >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}, std::vector < int16_t > {1, 2, 3}};
    map_int32_vector_vector_int16[4] = std::vector < std::vector < int16_t
            >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}, std::vector < int16_t > {1, 2, 3},
                std::vector < int16_t > {1, 2, 3, 4}};
    map_int32_vector_vector_int16[5] = std::vector < std::vector < int16_t
            >> {std::vector < int16_t > {1}, std::vector < int16_t > {1, 2}, std::vector < int16_t > {1, 2, 3},
                std::vector < int16_t > {1, 2, 3, 4}, std::vector < int16_t > {1, 2, 3, 4, 5}};
    map_int32_vector_set_int16.clear();
    map_int32_vector_set_int16[1] = std::vector < std::set < int16_t >> {std::set < int16_t > {1}};
    map_int32_vector_set_int16[2] =
            std::vector < std::set < int16_t >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}};
    map_int32_vector_set_int16[3] = std::vector < std::set < int16_t
            >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}, std::set < int16_t > {1, 2, 3}};
    map_int32_vector_set_int16[4] = std::vector < std::set < int16_t
            >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}, std::set < int16_t > {1, 2, 3},
                std::set < int16_t > {1, 2, 3, 4}};
    map_int32_vector_set_int16[5] = std::vector < std::set < int16_t
            >> {std::set < int16_t > {1}, std::set < int16_t > {1, 2}, std::set < int16_t > {1, 2, 3},
                std::set < int16_t > {1, 2, 3, 4}, std::set < int16_t > {1, 2, 3, 4, 5}};
    map_string_string.clear();
    map_string_string["one"] = "ONE";
    map_string_string["two"] = "TWO";
    map_string_string["three"] = "THREE";
    map_string_string["four"] = "FOUR";
    map_string_string["five"] = "FIVE";
    map_string_tstring.clear();
    map_string_tstring["one"] = "ONE";
    map_string_tstring["two"] = "TWO";
    map_string_tstring["three"] = "THREE";
    map_string_tstring["four"] = "FOUR";
    map_string_tstring["five"] = "FIVE";

    tree->Fill();

    tree->Write();
    file->Close();
}
