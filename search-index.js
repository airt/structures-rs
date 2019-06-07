var N=null,E="",T="t",U="u",searchIndex={};
var R=["structures","option","vertices","vertex","weight","hashmap","is_empty","usize","traverseorder","structures::doublylist","into_iter","try_from","borrow_mut","try_into","structures::graph","result","type_id","structures::heap","structures::list","borrow","typeid","structures::lru","structures::tree","intoiterator","default","formatter","from_iter","DoublyList","LruCache","TraverseOrder","IntoIter"];

searchIndex[R[0]]={"doc":E,"i":[[0,"doublylist",R[0],E,N,N],[3,R[27],R[9],"Doubly Linked List",N,N],[3,R[30],E,E,N,N],[11,"new",E,E,0,[[],["self"]]],[11,R[6],E,E,0,[[["self"]],["bool"]]],[11,"len",E,E,0,[[["self"]],[R[7]]]],[11,"front",E,E,0,[[["self"]],[[R[1],["ref"]],["ref"]]]],[11,"back",E,E,0,[[["self"]],[[R[1],["ref"]],["ref"]]]],[11,"push_front",E,E,0,[[["self"],[T]]]],[11,"push_back",E,E,0,[[["self"],[T]]]],[11,"pop_front",E,E,0,[[["self"]],[R[1]]]],[11,"pop_back",E,E,0,[[["self"]],[R[1]]]],[0,"graph",R[0],E,N,N],[3,"Graph",R[14],"Directed Graph",N,N],[6,"Weight",E,E,N,N],[6,"Vertex",E,E,N,N],[6,"Vertices",E,E,N,N],[6,"Edge",E,E,N,N],[6,"Edges",E,E,N,N],[6,"Path",E,E,N,N],[11,"new",E,E,1,[[],["self"]]],[11,R[2],E,E,1,[[["self"]],[R[2]]]],[11,"vertices_outgoing_from",E,E,1,[[[R[3]],["self"]],[R[2]]]],[11,"edges",E,E,1,[[["self"]],["edges"]]],[11,R[4],E,E,1,[[[R[3]],["self"]],[[R[4]],[R[1],[R[4]]]]]],[11,"add_edge",E,E,1,[[[R[3]],["self"],[R[4]]]]],[11,"shortest_path",E,"Dijkstra's Algorithm",1,[[[R[3]],[R[1],[R[3]]],["self"]],[[R[3]],[R[5],[R[3]]]]]],[11,"shortest_paths",E,"Floyd-Warshall Algorithm",1,[[["self"]],[[R[5],[R[3],R[5]]],[R[3]],[R[5],[R[3]]]]]],[11,"topo_sort",E,"Topological Sorting",1,[[["self"]],[[R[1],["path"]],["path"]]]],[0,"heap",R[0],E,N,N],[3,"Heap",R[17],"Max Heap",N,N],[11,"new",E,E,2,[[],["self"]]],[11,R[6],E,E,2,[[["self"]],["bool"]]],[11,"len",E,E,2,[[["self"]],[R[7]]]],[11,"peek",E,E,2,[[["self"]],[[R[1]],[T]]]],[11,"push",E,E,2,[[["self"],[T]]]],[11,"pop",E,E,2,[[["self"]],[R[1]]]],[11,"into_vec",E,E,2,[[],["vec"]]],[11,"into_sorted_vec",E,E,2,[[],["vec"]]],[0,"list",R[0],E,N,N],[3,"List",R[18],"Linked List",N,N],[11,"nil",E,E,3,[[],["self"]]],[11,"cons",E,E,3,[[["self"],[T]],["self"]]],[11,"decons",E,E,3,[[["self"]],[R[1]]]],[11,"head",E,E,3,[[["self"]],[[R[1]],[T]]]],[11,"tail",E,E,3,[[["self"]],[R[1]]]],[11,R[6],E,E,3,[[["self"]],["bool"]]],[11,"len",E,E,3,[[["self"]],[R[7]]]],[11,"iter",E,E,3,[[["self"]]]],[0,"lru",R[0],E,N,N],[3,R[28],R[21],"Least Recently Used Cache",N,N],[11,"with_capacity",E,E,4,[[[R[7]]],["self"]]],[11,R[6],E,E,4,[[["self"]],["bool"]]],[11,"len",E,E,4,[[["self"]],[R[7]]]],[11,"contains",E,E,4,[[["k"],["self"]],["bool"]]],[11,"peek",E,E,4,[[["k"],["self"]],[[R[1]],["v"]]]],[11,"get",E,E,4,[[["k"],["self"]],[[R[1]],["v"]]]],[11,"get_mut",E,E,4,[[["k"],["self"]],[["v"],[R[1]]]]],[11,"insert",E,E,4,[[["k"],["self"],["v"]],[R[1]]]],[11,"remove",E,E,4,[[["k"],["self"]],[R[1]]]],[0,"tree",R[0],E,N,N],[4,"Tree",R[22],"Binary Tree",N,N],[13,"Empty",E,E,5,N],[13,"Branch",E,E,5,N],[4,R[29],E,E,N,N],[13,"InOrder",E,E,6,N],[13,"PreOrder",E,E,6,N],[13,"PostOrder",E,E,6,N],[11,"empty",E,E,5,[[],["self"]]],[11,"leaf",E,E,5,[[[T]],["self"]]],[11,"branch",E,E,5,[[[T]],["self"]]],[11,"left",E,E,5,[[["self"]],[[R[1]],["self"]]]],[11,"right",E,E,5,[[["self"]],[[R[1]],["self"]]]],[11,"value",E,E,5,[[["self"]],[[R[1]],[T]]]],[11,"iter",E,E,5,[[[R[8]],["self"]]]],[11,"traverse",E,E,5,[[[R[8]],["fnmut"],["self"]]]],[14,"list",R[0],E,N,N],[11,"from",R[9],E,0,[[[T]],[T]]],[11,R[10],E,E,0,[[],["i"]]],[11,"into",E,E,0,[[],[U]]],[11,R[11],E,E,0,[[[U]],[R[15]]]],[11,R[19],E,E,0,[[["self"]],[T]]],[11,R[16],E,E,0,[[["self"]],[R[20]]]],[11,R[12],E,E,0,[[["self"]],[T]]],[11,R[13],E,E,0,[[],[R[15]]]],[11,"from",E,E,7,[[[T]],[T]]],[11,R[10],E,E,7,[[],["i"]]],[11,"into",E,E,7,[[],[U]]],[11,R[11],E,E,7,[[[U]],[R[15]]]],[11,R[19],E,E,7,[[["self"]],[T]]],[11,R[16],E,E,7,[[["self"]],[R[20]]]],[11,R[12],E,E,7,[[["self"]],[T]]],[11,R[13],E,E,7,[[],[R[15]]]],[11,"from",R[14],E,1,[[[T]],[T]]],[11,"into",E,E,1,[[],[U]]],[11,R[11],E,E,1,[[[U]],[R[15]]]],[11,R[19],E,E,1,[[["self"]],[T]]],[11,R[16],E,E,1,[[["self"]],[R[20]]]],[11,R[12],E,E,1,[[["self"]],[T]]],[11,R[13],E,E,1,[[],[R[15]]]],[11,"from",R[17],E,2,[[[T]],[T]]],[11,"into",E,E,2,[[],[U]]],[11,R[11],E,E,2,[[[U]],[R[15]]]],[11,R[19],E,E,2,[[["self"]],[T]]],[11,R[16],E,E,2,[[["self"]],[R[20]]]],[11,R[12],E,E,2,[[["self"]],[T]]],[11,R[13],E,E,2,[[],[R[15]]]],[11,"from",R[18],E,3,[[[T]],[T]]],[11,"into",E,E,3,[[],[U]]],[11,R[11],E,E,3,[[[U]],[R[15]]]],[11,R[19],E,E,3,[[["self"]],[T]]],[11,R[16],E,E,3,[[["self"]],[R[20]]]],[11,R[12],E,E,3,[[["self"]],[T]]],[11,R[13],E,E,3,[[],[R[15]]]],[11,"from",R[21],E,4,[[[T]],[T]]],[11,"into",E,E,4,[[],[U]]],[11,R[11],E,E,4,[[[U]],[R[15]]]],[11,R[19],E,E,4,[[["self"]],[T]]],[11,R[16],E,E,4,[[["self"]],[R[20]]]],[11,R[12],E,E,4,[[["self"]],[T]]],[11,R[13],E,E,4,[[],[R[15]]]],[11,"from",R[22],E,5,[[[T]],[T]]],[11,"into",E,E,5,[[],[U]]],[11,R[11],E,E,5,[[[U]],[R[15]]]],[11,R[19],E,E,5,[[["self"]],[T]]],[11,R[16],E,E,5,[[["self"]],[R[20]]]],[11,R[12],E,E,5,[[["self"]],[T]]],[11,R[13],E,E,5,[[],[R[15]]]],[11,"from",E,E,6,[[[T]],[T]]],[11,"into",E,E,6,[[],[U]]],[11,"to_owned",E,E,6,[[["self"]],[T]]],[11,"clone_into",E,E,6,[[[T],["self"]]]],[11,R[11],E,E,6,[[[U]],[R[15]]]],[11,R[19],E,E,6,[[["self"]],[T]]],[11,R[16],E,E,6,[[["self"]],[R[20]]]],[11,R[12],E,E,6,[[["self"]],[T]]],[11,R[13],E,E,6,[[],[R[15]]]],[11,"eq",R[18],E,3,[[["list"],["self"]],["bool"]]],[11,"ne",E,E,3,[[["list"],["self"]],["bool"]]],[11,"eq",R[22],E,5,[[["tree"],["self"]],["bool"]]],[11,"ne",E,E,5,[[["tree"],["self"]],["bool"]]],[11,"next_back",R[9],E,7,[[["self"]],[R[1]]]],[11,"clone",R[22],E,6,[[["self"]],[R[8]]]],[11,"from",R[17],E,2,[[["vec"]],["self"]]],[11,R[10],R[9],E,0,[[]]],[11,"extend",E,E,0,[[["self"],[R[23]]]]],[11,"extend",R[14],E,1,[[[R[23]],["self"]]]],[11,"drop",R[9],E,0,[[["self"]]]],[11,"drop",R[18],E,3,[[["self"]]]],[11,"next",R[9],E,7,[[["self"]],[R[1]]]],[11,R[24],E,E,0,[[],["self"]]],[11,R[24],R[14],E,1,[[],["graph"]]],[11,R[24],R[17],E,2,[[],["self"]]],[11,"fmt",R[18],E,3,[[[R[25]],["self"]],[R[15]]]],[11,"fmt",R[22],E,5,[[[R[25]],["self"]],[R[15]]]],[11,R[26],R[9],E,0,[[[R[23]]],["self"]]],[11,R[26],R[14],E,1,[[[R[23]]],["self"]]]],"p":[[3,R[27]],[3,"Graph"],[3,"Heap"],[3,"List"],[3,R[28]],[4,"Tree"],[4,R[29]],[3,R[30]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);