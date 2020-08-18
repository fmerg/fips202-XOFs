/// Secure left rotation
pub fn ROL(num: u64, offset: u8) -> u64 {
    (num << offset) ^ (num >> (64 - offset))
}

/// Loads a 64-bit value following the little-endian convention.
pub fn load64(x: &[u8]) -> u64 {
    let mut r: u64 = 0;
    for i in 0..8 {
        r |= (x[i] as u64) << 8 * i;
    }
    r
}

/// Stores a 64-bit value following the little-endian convention
pub fn store64(x: &mut [u8], u: &mut u64) {
    for i in 0..8 {
        x[i] = (*u) as u8;
        *u >>= 8;
    }
}


#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_ROL() {

        let mut parametrization: HashMap<(u64, u8), u64> = HashMap::new();

        parametrization.insert((0, 1), 0);
        parametrization.insert((0, 3), 0);
        parametrization.insert((0, 6), 0);
        parametrization.insert((0, 9), 0);
        parametrization.insert((0, 12), 0);
        parametrization.insert((0, 15), 0);
        parametrization.insert((0, 18), 0);
        parametrization.insert((0, 21), 0);
        parametrization.insert((0, 24), 0);
        parametrization.insert((0, 27), 0);
        parametrization.insert((0, 30), 0);
        parametrization.insert((0, 33), 0);
        parametrization.insert((0, 36), 0);
        parametrization.insert((0, 39), 0);
        parametrization.insert((0, 42), 0);
        parametrization.insert((0, 45), 0);
        parametrization.insert((0, 48), 0);
        parametrization.insert((0, 51), 0);
        parametrization.insert((0, 54), 0);
        parametrization.insert((0, 57), 0);
        parametrization.insert((0, 60), 0);
        parametrization.insert((0, 63), 0);
        parametrization.insert((18446744073709551615, 1), 18446744073709551615);
        parametrization.insert((18446744073709551615, 3), 18446744073709551615);
        parametrization.insert((18446744073709551615, 6), 18446744073709551615);
        parametrization.insert((18446744073709551615, 9), 18446744073709551615);
        parametrization.insert((18446744073709551615, 12), 18446744073709551615);
        parametrization.insert((18446744073709551615, 15), 18446744073709551615);
        parametrization.insert((18446744073709551615, 18), 18446744073709551615);
        parametrization.insert((18446744073709551615, 21), 18446744073709551615);
        parametrization.insert((18446744073709551615, 24), 18446744073709551615);
        parametrization.insert((18446744073709551615, 27), 18446744073709551615);
        parametrization.insert((18446744073709551615, 30), 18446744073709551615);
        parametrization.insert((18446744073709551615, 33), 18446744073709551615);
        parametrization.insert((18446744073709551615, 36), 18446744073709551615);
        parametrization.insert((18446744073709551615, 39), 18446744073709551615);
        parametrization.insert((18446744073709551615, 42), 18446744073709551615);
        parametrization.insert((18446744073709551615, 45), 18446744073709551615);
        parametrization.insert((18446744073709551615, 48), 18446744073709551615);
        parametrization.insert((18446744073709551615, 51), 18446744073709551615);
        parametrization.insert((18446744073709551615, 54), 18446744073709551615);
        parametrization.insert((18446744073709551615, 57), 18446744073709551615);
        parametrization.insert((18446744073709551615, 60), 18446744073709551615);
        parametrization.insert((18446744073709551615, 63), 18446744073709551615);
        parametrization.insert((10098092545668008332, 1), 1749441017626465049);
        parametrization.insert((10098092545668008332, 3), 6997764070505860196);
        parametrization.insert((10098092545668008332, 6), 641880342918226723);
        parametrization.insert((10098092545668008332, 9), 5135042743345813784);
        parametrization.insert((10098092545668008332, 12), 4186853799347407042);
        parametrization.insert((10098092545668008332, 15), 15048086321069704721);
        parametrization.insert((10098092545668008332, 18), 9704226126300328078);
        parametrization.insert((10098092545668008332, 21), 3846832715564418164);
        parametrization.insert((10098092545668008332, 24), 12327917650805793697);
        parametrization.insert((10098092545668008332, 27), 6389620837898591501);
        parametrization.insert((10098092545668008332, 30), 14223478555769628778);
        parametrization.insert((10098092545668008332, 33), 3107364003899720534);
        parametrization.insert((10098092545668008332, 36), 6412167957488212657);
        parametrization.insert((10098092545668008332, 39), 14403855512486598026);
        parametrization.insert((10098092545668008332, 42), 4550379657635474518);
        parametrization.insert((10098092545668008332, 45), 17956293187374244529);
        parametrization.insert((10098092545668008332, 48), 14523136983027094927);
        parametrization.insert((10098092545668008332, 51), 5504631421959449726);
        parametrization.insert((10098092545668008332, 54), 7143563228256494578);
        parametrization.insert((10098092545668008332, 57), 1808273604923301779);
        parametrization.insert((10098092545668008332, 60), 14466188839386414232);
        parametrization.insert((10098092545668008332, 63), 5049046272834004166);
        parametrization.insert((9692235910783506869, 1), 937727747857462123);
        parametrization.insert((9692235910783506869, 3), 3750910991429848492);
        parametrization.insert((9692235910783506869, 6), 11560543857729236321);
        parametrization.insert((9692235910783506869, 9), 250630493286132493);
        parametrization.insert((9692235910783506869, 12), 2005043946289059944);
        parametrization.insert((9692235910783506869, 15), 16040351570312479552);
        parametrization.insert((9692235910783506869, 18), 17642348120242526726);
        parametrization.insert((9692235910783506869, 21), 12011576445973352503);
        parametrization.insert((9692235910783506869, 24), 3858891199239061949);
        parametrization.insert((9692235910783506869, 27), 12424385520202943977);
        parametrization.insert((9692235910783506869, 30), 7161363793075793741);
        parametrization.insert((9692235910783506869, 33), 1950678123477695083);
        parametrization.insert((9692235910783506869, 36), 15605424987821560664);
        parametrization.insert((9692235910783506869, 39), 14162935460315175622);
        parametrization.insert((9692235910783506869, 42), 2623019240264095286);
        parametrization.insert((9692235910783506869, 45), 2537409848403210673);
        parametrization.insert((9692235910783506869, 48), 1852534713516133769);
        parametrization.insert((9692235910783506869, 51), 14820277708129070152);
        parametrization.insert((9692235910783506869, 54), 7881757222775251526);
        parametrization.insert((9692235910783506869, 57), 7713825561073357363);
        parametrization.insert((9692235910783506869, 60), 6370372267458204059);
        parametrization.insert((9692235910783506869, 63), 14069489992246529242);
        parametrization.insert((6741596070513288187, 1), 13483192141026576374);
        parametrization.insert((6741596070513288187, 3), 17039280416687202266);
        parametrization.insert((6741596070513288187, 6), 7187034817530756823);
        parametrization.insert((6741596070513288187, 9), 2156046319117399739);
        parametrization.insert((6741596070513288187, 12), 17248370552939197912);
        parametrization.insert((6741596070513288187, 15), 8859755907546721991);
        parametrization.insert((6741596070513288187, 18), 15537815039245121083);
        parametrization.insert((6741596070513288187, 21), 13622055871703658974);
        parametrization.insert((6741596070513288187, 24), 16742726605081513717);
        parametrization.insert((6741596070513288187, 27), 4814604324685248431);
        parametrization.insert((6741596070513288187, 30), 1623346450062884218);
        parametrization.insert((6741596070513288187, 33), 12986771600503073744);
        parametrization.insert((6741596070513288187, 36), 11660452435476831877);
        parametrization.insert((6741596070513288187, 39), 1049899115266896941);
        parametrization.insert((6741596070513288187, 42), 8399192922135175528);
        parametrization.insert((6741596070513288187, 45), 11853311155952749379);
        parametrization.insert((6741596070513288187, 48), 2592768879074236957);
        parametrization.insert((6741596070513288187, 51), 2295406958884344041);
        parametrization.insert((6741596070513288187, 54), 18363255671074752328);
        parametrization.insert((6741596070513288187, 57), 17778836852631157319);
        parametrization.insert((6741596070513288187, 60), 13103486305082397247);
        parametrization.insert((6741596070513288187, 63), 12594170072111419901);
        parametrization.insert((14170960180096846782, 1), 9895176286484141949);
        parametrization.insert((14170960180096846782, 3), 2687216998517464566);
        parametrization.insert((14170960180096846782, 6), 3050991914430164913);
        parametrization.insert((14170960180096846782, 9), 5961191241731767689);
        parametrization.insert((14170960180096846782, 12), 10796041786435038282);
        parametrization.insert((14170960180096846782, 15), 12581357996642099796);
        parametrization.insert((14170960180096846782, 18), 8417143604589040293);
        parametrization.insert((14170960180096846782, 21), 11996916615583667499);
        parametrization.insert((14170960180096846782, 24), 3741612556121581917);
        parametrization.insert((14170960180096846782, 27), 11486156375263103721);
        parametrization.insert((14170960180096846782, 30), 18102274707266623308);
        parametrization.insert((14170960180096846782, 33), 15690989142166125159);
        parametrization.insert((14170960180096846782, 36), 14847448695071691582);
        parametrization.insert((14170960180096846782, 39), 8099125118316222966);
        parametrization.insert((14170960180096846782, 42), 9452768725401128883);
        parametrization.insert((14170960180096846782, 45), 1835173508370824604);
        parametrization.insert((14170960180096846782, 48), 14681388066966596832);
        parametrization.insert((14170960180096846782, 51), 6770640093475464966);
        parametrization.insert((14170960180096846782, 54), 17271632600384616498);
        parametrization.insert((14170960180096846782, 57), 9045852287110070679);
        parametrization.insert((14170960180096846782, 60), 17026586075751910587);
        parametrization.insert((14170960180096846782, 63), 7085480090048423391);
        parametrization.insert((4140479544831169907, 1), 8280959089662339814);
        parametrization.insert((4140479544831169907, 3), 14677092284939807641);
        parametrization.insert((4140479544831169907, 6), 6736273837261151438);
        parametrization.insert((4140479544831169907, 9), 16996702550670108274);
        parametrization.insert((4140479544831169907, 12), 6846411889394004887);
        parametrization.insert((4140479544831169907, 15), 17877806967732935866);
        parametrization.insert((4140479544831169907, 18), 13895247225896625623);
        parametrization.insert((4140479544831169907, 21), 481513364915695294);
        parametrization.insert((4140479544831169907, 24), 3852106919325562352);
        parametrization.insert((4140479544831169907, 27), 12370111280894947201);
        parametrization.insert((4140479544831169907, 30), 6727169878611819533);
        parametrization.insert((4140479544831169907, 33), 16923870881475453034);
        parametrization.insert((4140479544831169907, 36), 6263758535836762967);
        parametrization.insert((4140479544831169907, 39), 13216580139275000506);
        parametrization.insert((4140479544831169907, 42), 13498920745652245973);
        parametrization.insert((4140479544831169907, 45), 15757645596670209709);
        parametrization.insert((4140479544831169907, 48), 15380700331104367982);
        parametrization.insert((4140479544831169907, 51), 12365138206577634166);
        parametrization.insert((4140479544831169907, 54), 6687385284073315253);
        parametrization.insert((4140479544831169907, 57), 16605594125167418794);
        parametrization.insert((4140479544831169907, 60), 3717544485372489047);
        parametrization.insert((4140479544831169907, 63), 11293611809270360761);
        parametrization.insert((12855064194353446388, 1), 7263384314997341161);
        parametrization.insert((12855064194353446388, 3), 10606793186279813029);
        parametrization.insert((12855064194353446388, 6), 11067369195400297772);
        parametrization.insert((12855064194353446388, 9), 14751977268364175716);
        parametrization.insert((12855064194353446388, 12), 7335353704656096038);
        parametrization.insert((12855064194353446388, 15), 3342597416120113459);
        parametrization.insert((12855064194353446388, 18), 8294035255251356057);
        parametrization.insert((12855064194353446388, 21), 11012049820882193611);
        parametrization.insert((12855064194353446388, 24), 14309422272219342428);
        parametrization.insert((12855064194353446388, 27), 3794913735497429734);
        parametrization.insert((12855064194353446388, 30), 11912565810269886257);
        parametrization.insert((12855064194353446388, 33), 3066806113611331981);
        parametrization.insert((12855064194353446388, 36), 6087704835181104233);
        parametrization.insert((12855064194353446388, 39), 11808150534029730634);
        parametrization.insert((12855064194353446388, 42), 2231483903690086997);
        parametrization.insert((12855064194353446388, 45), 17851871229520695976);
        parametrization.insert((12855064194353446388, 48), 13687761320198706503);
        parametrization.insert((12855064194353446388, 51), 17268370193041893949);
        parametrization.insert((12855064194353446388, 54), 9019753028368290287);
        parametrization.insert((12855064194353446388, 57), 16817792005817667451);
        parametrization.insert((12855064194353446388, 60), 5415127530574478303);
        parametrization.insert((12855064194353446388, 63), 6427532097176723194);
        parametrization.insert((10930136465080546117, 1), 3413528856451540619);
        parametrization.insert((10930136465080546117, 3), 13654115425806162476);
        parametrization.insert((10930136465080546117, 6), 16999203037901541733);
        parametrization.insert((10930136465080546117, 9), 6866415787245472559);
        parametrization.insert((10930136465080546117, 12), 18037838150544677242);
        parametrization.insert((10930136465080546117, 15), 15175496688390556631);
        parametrization.insert((10930136465080546117, 18), 10723509064867143358);
        parametrization.insert((10930136465080546117, 21), 12001096224098940404);
        parametrization.insert((10930136465080546117, 24), 3775049424243765157);
        parametrization.insert((10930136465080546117, 27), 11753651320240569641);
        parametrization.insert((10930136465080546117, 30), 1795490193376799053);
        parametrization.insert((10930136465080546117, 33), 14363921547014392424);
        parametrization.insert((10930136465080546117, 36), 4230907933857829702);
        parametrization.insert((10930136465080546117, 39), 15400519397153086001);
        parametrization.insert((10930136465080546117, 42), 12523690734967378318);
        parametrization.insert((10930136465080546117, 45), 7955805511191268469);
        parametrization.insert((10930136465080546117, 48), 8306211868401492907);
        parametrization.insert((10930136465080546117, 51), 11109462726083288411);
        parametrization.insert((10930136465080546117, 54), 15088725513828100828);
        parametrization.insert((10930136465080546117, 57), 10029339668367496934);
        parametrization.insert((10930136465080546117, 60), 6447741052101769012);
        parametrization.insert((10930136465080546117, 63), 14688440269395048866);
        parametrization.insert((8927074340688041762, 1), 17854148681376083524);
        parametrization.insert((8927074340688041762, 3), 16076362504375679251);
        parametrization.insert((8927074340688041762, 6), 17930435592748124318);
        parametrization.insert((8927074340688041762, 9), 14316276226018133239);
        parametrization.insert((8927074340688041762, 12), 3849745365887756222);
        parametrization.insert((8927074340688041762, 15), 12351218853392498161);
        parametrization.insert((8927074340688041762, 18), 6576030458592227213);
        parametrization.insert((8927074340688041762, 21), 15714755521318714474);
        parametrization.insert((8927074340688041762, 24), 15037579728292406102);
        parametrization.insert((8927074340688041762, 27), 9620173384081939126);
        parametrization.insert((8927074340688041762, 30), 3174410777817306548);
        parametrization.insert((8927074340688041762, 33), 6948542148828900769);
        parametrization.insert((8927074340688041762, 36), 248104969502551307);
        parametrization.insert((8927074340688041762, 39), 1984839756020410456);
        parametrization.insert((8927074340688041762, 42), 15878718048163283648);
        parametrization.insert((8927074340688041762, 45), 16349279943048959494);
        parametrization.insert((8927074340688041762, 48), 1667031028424814647);
        parametrization.insert((8927074340688041762, 51), 13336248227398517176);
        parametrization.insert((8927074340688041762, 54), 14456265450640379333);
        parametrization.insert((8927074340688041762, 57), 4969659162865724974);
        parametrization.insert((8927074340688041762, 60), 2863785155506696562);
        parametrization.insert((8927074340688041762, 63), 4463537170344020881);
        parametrization.insert((8247140940023939357, 1), 16494281880047878714);
        parametrization.insert((8247140940023939357, 3), 10636895299062860011);
        parametrization.insert((8247140940023939357, 6), 11308186097664673628);
        parametrization.insert((8247140940023939357, 9), 16678512486479182564);
        parametrization.insert((8247140940023939357, 12), 4300891375866599207);
        parametrization.insert((8247140940023939357, 15), 15960386933223242041);
        parametrization.insert((8247140940023939357, 18), 17002631023528626638);
        parametrization.insert((8247140940023939357, 21), 6893839672262151799);
        parametrization.insert((8247140940023939357, 24), 18257229230678111162);
        parametrization.insert((8247140940023939357, 27), 16930625329458027991);
        parametrization.insert((8247140940023939357, 30), 6317794119697362623);
        parametrization.insert((8247140940023939357, 33), 13648864810159797754);
        parametrization.insert((8247140940023939357, 36), 16957198112730623957);
        parametrization.insert((8247140940023939357, 39), 6530376385878130351);
        parametrization.insert((8247140940023939357, 42), 15349522939605939578);
        parametrization.insert((8247140940023939357, 45), 12115719074590206934);
        parametrization.insert((8247140940023939357, 48), 4692032228173897397);
        parametrization.insert((8247140940023939357, 51), 642769677972075946);
        parametrization.insert((8247140940023939357, 54), 5142157423776607568);
        parametrization.insert((8247140940023939357, 57), 4243771242793757314);
        parametrization.insert((8247140940023939357, 60), 15503425868640506897);
        parametrization.insert((8247140940023939357, 63), 13346942506866745486);
        parametrization.insert((1553066054745300745, 1), 3106132109490601490);
        parametrization.insert((1553066054745300745, 3), 12424528437962405960);
        parametrization.insert((1553066054745300745, 6), 7162507135151489605);
        parametrization.insert((1553066054745300745, 9), 1959824860083261995);
        parametrization.insert((1553066054745300745, 12), 15678598880666095960);
        parametrization.insert((1553066054745300745, 15), 14748326603071457990);
        parametrization.insert((1553066054745300745, 18), 7306148382314354230);
        parametrization.insert((1553066054745300745, 21), 3108954837386178995);
        parametrization.insert((1553066054745300745, 24), 6424894625379880345);
        parametrization.insert((1553066054745300745, 27), 14505668855619939530);
        parametrization.insert((1553066054745300745, 30), 5364886402702206550);
        parametrization.insert((1553066054745300745, 33), 6025603074198549170);
        parametrization.insert((1553066054745300745, 36), 11311336446169290130);
        parametrization.insert((1553066054745300745, 39), 16703715274516114580);
        parametrization.insert((1553066054745300745, 42), 4502513680162055335);
        parametrization.insert((1553066054745300745, 45), 17573365367586891065);
        parametrization.insert((1553066054745300745, 48), 11459714424728267215);
        parametrization.insert((1553066054745300745, 51), 17890739102987931260);
        parametrization.insert((1553066054745300745, 54), 13998704307936588775);
        parametrization.insert((1553066054745300745, 57), 1309170021235400510);
        parametrization.insert((1553066054745300745, 60), 10473360169883204080);
        parametrization.insert((1553066054745300745, 63), 9999905064227426180);
        parametrization.insert((5775566247763292545, 1), 11551132495526585090);
        parametrization.insert((5775566247763292545, 3), 9311041834687237130);
        parametrization.insert((5775566247763292545, 6), 701358382659690580);
        parametrization.insert((5775566247763292545, 9), 5610867061277524640);
        parametrization.insert((5775566247763292545, 12), 7993448342801093890);
        parametrization.insert((5775566247763292545, 15), 8607354521280096275);
        parametrization.insert((5775566247763292545, 18), 13518603949112115355);
        parametrization.insert((5775566247763292545, 21), 15915111224349164765);
        parametrization.insert((5775566247763292545, 24), 16640425352536008430);
        parametrization.insert((5775566247763292545, 27), 3996194304321206135);
        parametrization.insert((5775566247763292545, 30), 13522810360860097465);
        parametrization.insert((5775566247763292545, 33), 15948762518333021645);
        parametrization.insert((5775566247763292545, 36), 16909635704406863470);
        parametrization.insert((5775566247763292545, 39), 6149877119288046455);
        parametrization.insert((5775566247763292545, 42), 12305528806885268410);
        parametrization.insert((5775566247763292545, 45), 6210510086534389205);
        parametrization.insert((5775566247763292545, 48), 12790592544856010410);
        parametrization.insert((5775566247763292545, 51), 10091019990300325205);
        parametrization.insert((5775566247763292545, 54), 6941183627564395180);
        parametrization.insert((5775566247763292545, 57), 189236799386506595);
        parametrization.insert((5775566247763292545, 60), 1513894395092052760);
        parametrization.insert((5775566247763292545, 63), 12111155160736422080);
        parametrization.insert((4649639024836382244, 1), 9299278049672764488);
        parametrization.insert((4649639024836382244, 3), 303624051271954722);
        parametrization.insert((4649639024836382244, 6), 2428992410175637776);
        parametrization.insert((4649639024836382244, 9), 985195207695550593);
        parametrization.insert((4649639024836382244, 12), 7881561661564404744);
        parametrization.insert((4649639024836382244, 15), 7712261071386583107);
        parametrization.insert((4649639024836382244, 18), 6357856349964010011);
        parametrization.insert((4649639024836382244, 21), 13969362652292976858);
        parametrization.insert((4649639024836382244, 24), 1074436776086505174);
        parametrization.insert((4649639024836382244, 27), 8595494208692041392);
        parametrization.insert((4649639024836382244, 30), 13423721448407676291);
        parametrization.insert((4649639024836382244, 33), 15156051218713652253);
        parametrization.insert((4649639024836382244, 36), 10567945307451908334);
        parametrization.insert((4649639024836382244, 39), 10756586164777060212);
        parametrization.insert((4649639024836382244, 42), 12265713023378275236);
        parametrization.insert((4649639024836382244, 45), 5891983818478443813);
        parametrization.insert((4649639024836382244, 48), 10242382400408447274);
        parametrization.insert((4649639024836382244, 51), 8152082908429371732);
        parametrization.insert((4649639024836382244, 54), 9876431046306319011);
        parametrization.insert((4649639024836382244, 57), 5224472075612345628);
        parametrization.insert((4649639024836382244, 60), 4902288457479661794);
        parametrization.insert((4649639024836382244, 63), 2324819512418191122);
        parametrization.insert((17515159986510223484, 1), 16583575899310895353);
        parametrization.insert((17515159986510223484, 3), 10994071376114926567);
        parametrization.insert((17515159986510223484, 6), 14165594714081206076);
        parametrization.insert((17515159986510223484, 9), 2644293270392338918);
        parametrization.insert((17515159986510223484, 12), 2707602089429159729);
        parametrization.insert((17515159986510223484, 15), 3214072641723726217);
        parametrization.insert((17515159986510223484, 18), 7265837060080258121);
        parametrization.insert((17515159986510223484, 21), 2786464259513410123);
        parametrization.insert((17515159986510223484, 24), 3844970002397729369);
        parametrization.insert((17515159986510223484, 27), 12313015945472283337);
        parametrization.insert((17515159986510223484, 30), 6270407195230508621);
        parametrization.insert((17515159986510223484, 33), 13269769414424965738);
        parametrization.insert((17515159986510223484, 36), 13924434946851967829);
        parametrization.insert((17515159986510223484, 39), 715015132558432942);
        parametrization.insert((17515159986510223484, 42), 5720121060467463536);
        parametrization.insert((17515159986510223484, 45), 8867480336320605058);
        parametrization.insert((17515159986510223484, 48), 15599610469436185619);
        parametrization.insert((17515159986510223484, 51), 14116419313232175262);
        parametrization.insert((17515159986510223484, 54), 2250890063600092406);
        parametrization.insert((17515159986510223484, 57), 18007120508800739248);
        parametrization.insert((17515159986510223484, 60), 14929755554439052679);
        parametrization.insert((17515159986510223484, 63), 8757579993255111742);
        parametrization.insert((10518223924126276283, 1), 2589703774543000951);
        parametrization.insert((10518223924126276283, 3), 10358815098172003804);
        parametrization.insert((10518223924126276283, 6), 9083544490537823972);
        parametrization.insert((10518223924126276283, 9), 17328123703173936931);
        parametrization.insert((10518223924126276283, 12), 9497781109424634143);
        parametrization.insert((10518223924126276283, 15), 2195272580558866684);
        parametrization.insert((10518223924126276283, 18), 17562180644470933472);
        parametrization.insert((10518223924126276283, 21), 11370236639800606471);
        parametrization.insert((10518223924126276283, 24), 17174916823566645308);
        parametrization.insert((10518223924126276283, 27), 8272126072566301159);
        parametrization.insert((10518223924126276283, 30), 10836776359401754427);
        parametrization.insert((10518223924126276283, 33), 12907234580375828956);
        parametrization.insert((10518223924126276283, 36), 11024156274458873573);
        parametrization.insert((10518223924126276283, 39), 14406273900832782124);
        parametrization.insert((10518223924126276283, 42), 4569726764404947302);
        parametrization.insert((10518223924126276283, 45), 18111070041530026801);
        parametrization.insert((10518223924126276283, 48), 15761351816273353103);
        parametrization.insert((10518223924126276283, 51), 15410350087929515134);
        parametrization.insert((10518223924126276283, 54), 12602336261178811382);
        parametrization.insert((10518223924126276283, 57), 8584969720882732981);
        parametrization.insert((10518223924126276283, 60), 13339525545933209003);
        parametrization.insert((10518223924126276283, 63), 14482483998917913949);
        parametrization.insert((584334390396558241, 1), 1168668780793116482);
        parametrization.insert((584334390396558241, 3), 4674675123172465928);
        parametrization.insert((584334390396558241, 6), 503912837960624194);
        parametrization.insert((584334390396558241, 9), 4031302703684993552);
        parametrization.insert((584334390396558241, 12), 13803677555770396801);
        parametrization.insert((584334390396558241, 15), 18195700077615416333);
        parametrization.insert((584334390396558241, 18), 16438392104956469359);
        parametrization.insert((584334390396558241, 21), 2379928323684893567);
        parametrization.insert((584334390396558241, 24), 592682515769596921);
        parametrization.insert((584334390396558241, 27), 4741460126156775368);
        parametrization.insert((584334390396558241, 30), 1038192861835099714);
        parametrization.insert((584334390396558241, 33), 8305542894680797712);
        parametrization.insert((584334390396558241, 36), 11104110936317726851);
        parametrization.insert((584334390396558241, 39), 15045911195703608348);
        parametrization.insert((584334390396558241, 42), 9686825123371557094);
        parametrization.insert((584334390396558241, 45), 3707624692134250292);
        parametrization.insert((584334390396558241, 48), 11214253463364450721);
        parametrization.insert((584334390396558241, 51), 15927051412077399308);
        parametrization.insert((584334390396558241, 54), 16735946854361884774);
        parametrization.insert((584334390396558241, 57), 4760366318928216887);
        parametrization.insert((584334390396558241, 60), 1189442404006631866);
        parametrization.insert((584334390396558241, 63), 9515539232053054928);
        parametrization.insert((4673820999500469152, 1), 9347641999000938304);
        parametrization.insert((4673820999500469152, 3), 497079848584649986);
        parametrization.insert((4673820999500469152, 6), 3976638788677199888);
        parametrization.insert((4673820999500469152, 9), 13366366235708047489);
        parametrization.insert((4673820999500469152, 12), 14697209517116621837);
        parametrization.insert((4673820999500469152, 15), 6897211694675665006);
        parametrization.insert((4673820999500469152, 18), 18284205409986216818);
        parametrization.insert((4673820999500469152, 21), 17146434763922873239);
        parametrization.insert((4673820999500469152, 24), 8044269595416124607);
        parametrization.insert((4673820999500469152, 27), 9013924542200342011);
        parametrization.insert((4673820999500469152, 30), 16771164116474081243);
        parametrization.insert((4673820999500469152, 33), 5042104415825788639);
        parametrization.insert((4673820999500469152, 36), 3443347179187205882);
        parametrization.insert((4673820999500469152, 39), 9100033359788095441);
        parametrization.insert((4673820999500469152, 42), 17460034657176108683);
        parametrization.insert((4673820999500469152, 45), 10553068741442008159);
        parametrization.insert((4673820999500469152, 48), 10637573636697858812);
        parametrization.insert((4673820999500469152, 51), 11313612798744664036);
        parametrization.insert((4673820999500469152, 54), 16721926095119105828);
        parametrization.insert((4673820999500469152, 57), 4648200244985985319);
        parametrization.insert((4673820999500469152, 60), 292113812468779322);
        parametrization.insert((4673820999500469152, 63), 2336910499750234576);

        for ((num, offset), expected) in parametrization {
            let result: u64 = ROL(num, offset);
            assert_eq!(result, expected);
        }
    }


    #[test]
    fn test_load64() {

        let mut parametrization: HashMap<[u8; 8], u64> = HashMap::new();

        parametrization.insert([97, 98, 99, 100, 101, 102, 103, 104], 7523094288207667809);
        parametrization.insert([32, 33, 34, 35, 36, 37, 38, 39], 2820983053732684064);
        parametrization.insert([40, 41, 42, 43, 44, 45, 46, 47], 3399704436437297448);
        parametrization.insert([48, 49, 50, 51, 52, 53, 54, 55], 3978425819141910832);
        parametrization.insert([56, 57, 58, 59, 60, 61, 62, 63], 4557147201846524216);
        parametrization.insert([64, 65, 66, 67, 68, 69, 70, 71], 5135868584551137600);
        parametrization.insert([72, 73, 74, 75, 76, 77, 78, 79], 5714589967255750984);
        parametrization.insert([80, 81, 82, 83, 84, 85, 86, 87], 6293311349960364368);
        parametrization.insert([88, 89, 90, 91, 92, 93, 94, 95], 6872032732664977752);
        parametrization.insert([96, 97, 98, 99, 100, 101, 102, 103], 7450754115369591136);
        parametrization.insert([104, 105, 106, 107, 108, 109, 110, 111], 8029475498074204520);
        parametrization.insert([112, 113, 114, 115, 116, 117, 118, 119], 8608196880778817904);
        parametrization.insert([120, 121, 122, 123, 124, 125, 126, 127], 9186918263483431288);

        for (_input, expected) in parametrization {
            let mut input: [u8; 8] = [0; 8];
            input.copy_from_slice(&_input);
            assert_eq!(load64(&mut input), expected);
        }
    }


    #[test]
    fn test_store64() {

        let mut parametrization: HashMap<u64, [u8; 8]> = HashMap::new();

        parametrization.insert(0, [0, 0, 0, 0, 0, 0, 0, 0]);
        parametrization.insert(1, [1, 0, 0, 0, 0, 0, 0, 0]);
        parametrization.insert(2, [2, 0, 0, 0, 0, 0, 0, 0]);
        parametrization.insert(6, [6, 0, 0, 0, 0, 0, 0, 0]);
        parametrization.insert(42, [42, 0, 0, 0, 0, 0, 0, 0]);
        parametrization.insert(1806, [14, 7, 0, 0, 0, 0, 0, 0]);
        parametrization.insert(3263442, [210, 203, 49, 0, 0, 0, 0, 0]);
        parametrization.insert(10650056950806, [22, 132, 220, 168, 175, 9, 0, 0]);
        parametrization.insert(999999999999999999, [255, 255, 99, 167, 179, 182, 224, 13]);

        for (u, expected) in parametrization {
            let mut input: [u8; 8] = [0; 8];
            let mut w: u64 = u;
            store64(&mut input, &mut w);
            assert_eq!(input, expected);
        }
    }

}