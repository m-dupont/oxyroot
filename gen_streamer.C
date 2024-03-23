void gen_streamer() {

  auto name = "TTree";

  cout << ";GetStreamerInfo name=" << name << endl;
  auto s = TClass::GetClass(name)->GetStreamerInfo();
  s->Dump();

  
  auto a = s->GetElements();
  cout << ";GetElements name=" << name << endl;
  a->Dump();

  exit(0);
}


