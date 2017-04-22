var searchIndex = {};
searchIndex["dynasm"] = {"doc":"","items":[[3,"State","dynasm","This struct contains all non-parsing state that dynasm! requires while parsing and compiling",null,null],[12,"stmts","","",0,null],[12,"target","","",0,null],[12,"crate_data","","",0,null],[3,"DynasmData","","",null,null],[12,"current_arch","","",1,null],[12,"aliases","","",1,null],[5,"registrar","","Welcome to the documentation of the dynasm plugin. This mostly exists to ease development and to show a glimpse of what is under the hood of dynasm. Please be aware that nothing in here should be counted on to be stable, the only guarantees are in the syntax the `dynasm!` macro parses and in the code it generates.",null,{"inputs":[{"name":"registry"}],"output":null}],[5,"crate_local_data","","",null,{"inputs":[{"name":"extctxt"}],"output":{"name":"cratelocaldata"}}],[0,"arch","","",null,null],[3,"DummyArch","dynasm::arch","",null,null],[5,"from_str","","",null,{"inputs":[{"name":"str"}],"output":{"name":"option"}}],[0,"x64","","",null,null],[3,"Archx64","dynasm::arch::x64","",null,null],[0,"debug","","",null,null],[5,"format_opdata_list","dynasm::arch::x64::debug","",null,null],[5,"format_opdata","","",null,{"inputs":[{"name":"str"},{"name":"opdata"}],"output":{"name":"vec"}}],[0,"x64data","dynasm::arch::x64","",null,null],[5,"get_mnemnonic_data","dynasm::arch::x64::x64data","",null,{"inputs":[{"name":"str"}],"output":{"name":"option"}}],[5,"mnemnonics","","",null,{"inputs":[],"output":{"name":"keys"}}],[0,"flags","","",null,null],[3,"Flags","dynasm::arch::x64::x64data::flags","",null,null],[5,"flag_bits","","",null,{"inputs":[{"name":"flags"}],"output":{"name":"u32"}}],[5,"make_flag","","",null,{"inputs":[{"name":"u32"}],"output":{"name":"flags"}}],[17,"DEFAULT","","",null,null],[17,"VEX_OP","","",null,null],[17,"XOP_OP","","",null,null],[17,"IMM_OP","","",null,null],[17,"AUTO_SIZE","","",null,null],[17,"AUTO_NO32","","",null,null],[17,"AUTO_REXW","","",null,null],[17,"AUTO_VEXL","","",null,null],[17,"WORD_SIZE","","",null,null],[17,"WITH_REXW","","",null,null],[17,"WITH_VEXL","","",null,null],[17,"EXACT_SIZE","","",null,null],[17,"PREF_66","","",null,null],[17,"PREF_67","","",null,null],[17,"PREF_F0","","",null,null],[17,"PREF_F2","","",null,null],[17,"PREF_F3","","",null,null],[17,"LOCK","","",null,null],[17,"REP","","",null,null],[17,"REPE","","",null,null],[17,"SHORT_ARG","","",null,null],[17,"ENC_MR","","",null,null],[17,"ENC_VM","","",null,null],[17,"ENC_MIB","","",null,null],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"flags"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"option"}}],[11,"lt","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"le","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"gt","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"ge","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"cmp","","",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"ordering"}}],[11,"hash","","",2,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"empty","","Returns an empty set of flags.",2,{"inputs":[],"output":{"name":"flags"}}],[11,"all","","Returns the set containing all flags.",2,{"inputs":[],"output":{"name":"flags"}}],[11,"bits","","Returns the raw value of the flags currently stored.",2,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"from_bits","","Convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag.",2,{"inputs":[{"name":"u32"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits that do not correspond to flags.",2,{"inputs":[{"name":"u32"}],"output":{"name":"flags"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",2,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_all","","Returns `true` if all flags are currently set.",2,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"insert","","Inserts the specified flags in-place.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"remove","","Removes the specified flags in-place.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"toggle","","Toggles the specified flags in-place.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"bitor","","Returns the union of the two sets of flags.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"bitor_assign","","Adds the set of flags.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"bitxor_assign","","Toggles the set of flags.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"bitand","","Returns the intersection between the two sets of flags.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"bitand_assign","","Disables all flags disabled in the set.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"sub","","Returns the set difference of the two sets of flags.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"sub_assign","","Disables all flags enabled in the set.",2,{"inputs":[{"name":"self"},{"name":"flags"}],"output":null}],[11,"not","","Returns the complement of this set of flags.",2,{"inputs":[{"name":"self"}],"output":{"name":"flags"}}],[11,"extend","","",2,{"inputs":[{"name":"self"},{"name":"t"}],"output":null}],[11,"from_iter","","",2,{"inputs":[{"name":"t"}],"output":{"name":"flags"}}],[0,"features","dynasm::arch::x64::x64data","",null,null],[3,"Features","dynasm::arch::x64::x64data::features","",null,null],[5,"flag_bits","","",null,{"inputs":[{"name":"features"}],"output":{"name":"u32"}}],[5,"make_flag","","",null,{"inputs":[{"name":"u32"}],"output":{"name":"features"}}],[17,"X64_IMPLICIT","","",null,null],[17,"FPU","","",null,null],[17,"MMX","","",null,null],[17,"TDNOW","","",null,null],[17,"SSE","","",null,null],[17,"SSE2","","",null,null],[17,"SSE3","","",null,null],[17,"VMX","","",null,null],[17,"SSSE3","","",null,null],[17,"SSE4A","","",null,null],[17,"SSE41","","",null,null],[17,"SSE42","","",null,null],[17,"SSE5","","",null,null],[17,"AVX","","",null,null],[17,"AVX2","","",null,null],[17,"FMA","","",null,null],[17,"BMI1","","",null,null],[17,"BMI2","","",null,null],[17,"TBM","","",null,null],[17,"RTM","","",null,null],[17,"INVPCID","","",null,null],[17,"MPX","","",null,null],[17,"SHA","","",null,null],[17,"PREFETCHWT1","","",null,null],[17,"CYRIX","","",null,null],[17,"AMD","","",null,null],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"features"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"ordering"}}],[11,"hash","","",3,null],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"empty","","Returns an empty set of flags.",3,{"inputs":[],"output":{"name":"features"}}],[11,"all","","Returns the set containing all flags.",3,{"inputs":[],"output":{"name":"features"}}],[11,"bits","","Returns the raw value of the flags currently stored.",3,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"from_bits","","Convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag.",3,{"inputs":[{"name":"u32"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits that do not correspond to flags.",3,{"inputs":[{"name":"u32"}],"output":{"name":"features"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",3,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_all","","Returns `true` if all flags are currently set.",3,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"bool"}}],[11,"insert","","Inserts the specified flags in-place.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"remove","","Removes the specified flags in-place.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"toggle","","Toggles the specified flags in-place.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"bitor","","Returns the union of the two sets of flags.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"features"}}],[11,"bitor_assign","","Adds the set of flags.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"features"}}],[11,"bitxor_assign","","Toggles the set of flags.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"bitand","","Returns the intersection between the two sets of flags.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"features"}}],[11,"bitand_assign","","Disables all flags disabled in the set.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"sub","","Returns the set difference of the two sets of flags.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":{"name":"features"}}],[11,"sub_assign","","Disables all flags enabled in the set.",3,{"inputs":[{"name":"self"},{"name":"features"}],"output":null}],[11,"not","","Returns the complement of this set of flags.",3,{"inputs":[{"name":"self"}],"output":{"name":"features"}}],[11,"extend","","",3,{"inputs":[{"name":"self"},{"name":"t"}],"output":null}],[11,"from_iter","","",3,{"inputs":[{"name":"t"}],"output":{"name":"features"}}],[11,"clone","dynasm::arch::x64","",4,{"inputs":[{"name":"self"}],"output":{"name":"archx64"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",4,{"inputs":[],"output":{"name":"archx64"}}],[11,"name","","",4,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"set_features","","",4,null],[11,"compile_instruction","","",4,{"inputs":[{"name":"self"},{"name":"state"},{"name":"extctxt"},{"name":"parser"}],"output":{"name":"presult"}}],[17,"CURRENT_ARCH","dynasm::arch","",null,null],[8,"Arch","","",null,null],[10,"name","","",5,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[10,"set_features","","",5,null],[10,"compile_instruction","","",5,{"inputs":[{"name":"self"},{"name":"state"},{"name":"extctxt"},{"name":"parser"}],"output":{"name":"presult"}}],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"dummyarch"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"name","","",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"set_features","","",6,null],[11,"compile_instruction","","",6,{"inputs":[{"name":"self"},{"name":"state"},{"name":"extctxt"},{"name":"parser"}],"output":{"name":"presult"}}],[11,"evaluate_directive","dynasm","",1,{"inputs":[{"name":"self"},{"name":"vec"},{"name":"extctxt"},{"name":"parser"}],"output":{"name":"presult"}}],[6,"CrateLocalData","","",null,null]],"paths":[[3,"State"],[3,"DynasmData"],[3,"Flags"],[3,"Features"],[3,"Archx64"],[8,"Arch"],[3,"DummyArch"]]};
initSearch(searchIndex);
var path = $(".location").text();
var nest_count;
if (path) {
    nest_count = path.split("::").length + 1;
} else {
    nest_count = 1;
}

var base_path = "";
for (i = 0; i < nest_count; i++) {
    base_path += "../";
}

$(".sidebar").prepend('\
  <p class="location">\
      <a href="' + base_path + 'language/index.html">dynasm-rs</a>\
  </p>\
  <div class = "block modules">\
  <h3>Components</h3>\
    <ul>\
      <li>\
        <a href="' + base_path + 'language/index.html">Syntax</a>\
      </li>\
      <li>\
        <a href="' + base_path + 'plugin/dynasm/index.html">Plugin (dynasm)</a>\
      </li>\
      <li>\
        <a href="' + base_path + 'runtime/dynasmrt/index.html">Runtime (dynasmrt)</a>\
      </li>\
    </ul>\
  </div>');
