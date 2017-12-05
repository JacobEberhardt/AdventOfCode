mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Advent of Code Solutions");

// Day 1
    println!("Day 1: Reverse Captchas");

    println!("Star 1: Match next digit");
    let input: &str = "892195969991735837915273868729548694237967495115412399373194562526947585337233793568278265279199883197167634791293177986152566236718332617536487236879747167999983363832257912445756887314879229925864477761357139855548522513798899853896612387146687716264599943289416326727256525173953861534244979466587895429399159924916364476319573895566795393368411672387263615582128377676293612892723762237191146714286233543514411813323197995953854871628225358543514157867372265718724276911699514971458844849349726276329135118243155698271218844347387457343656446381799296893222256198484465873714311777937421161581798189554141474236239447612421883232173914183732126332838194648583472419154369952477422666389517569944428464617457124369349242479612422673241361777576466946622932243728551273284837934497511114334421486262244982914734452113946361245377351849815584855691778894798219822463298387771923329337634394654439458564233259451453345316753241438267739439225497515276522424441532462541528195782818326918562247278496495764435386667383577543385186827269732261223156824351164841648424564925198783625721396988984481558391866483955533972212164693898955412719161648411279149413443192896864258215498543827458438871355879336892721675937111952479183496982825163456282747678364612135596373533447719867384667516572262124225585623974278833981365494628646614588114147473559138853453189448624976774641922469183942857695986376428944876851497914443873513862319484181787593572987444669767939526294424531262999564948571142342741129862311311313166798363442745792896227642881893134498151552326647933689596516859342242244584714818773791567187322217164347852843751875979415198165627534263527828414549217234322361937785185174993256753483876378332521824515977173397535784236923629636713469151526399149548322849831431526219478653861754364155275865511643923249858589466142474763778413826829226663398467569555747267195129525138917561785436449855933951538973995881954521124753369223898312843734771532342383282987422334196585128526526324291777689689492346231786335851551413876834969878";
    println!("Solution: {}", day1::calculate_reverse_captcha_star1(input));

    println!("Star 2: Match digit at position + n/2");
    println!("Solution: {}", day1::calculate_reverse_captcha_star2(input));

// Day 2
    println!("Day 2: Corruption Checksum");

    let input: &str = "414	382	1515	319	83	1327	116	391	101	749	1388	1046	1427	105	1341	1590
    960	930	192	147	932	621	1139	198	865	820	597	165	232	417	19	183
    3379	987	190	3844	1245	1503	3151	3349	2844	4033	175	3625	3565	179	3938	184
    116	51	32	155	102	92	65	42	48	91	74	69	52	89	20	143
    221	781	819	121	821	839	95	117	626	127	559	803	779	543	44	369
    199	2556	93	1101	122	124	2714	625	2432	1839	2700	2636	118	2306	1616	2799
    56	804	52	881	1409	47	246	1368	1371	583	49	1352	976	400	1276	1240
    1189	73	148	1089	93	76	3205	3440	3627	92	853	95	3314	3551	2929	3626
    702	169	492	167	712	488	357	414	187	278	87	150	19	818	178	686
    140	2220	1961	1014	2204	2173	1513	2225	443	123	148	580	833	1473	137	245
    662	213	1234	199	1353	1358	1408	235	917	1395	1347	194	565	179	768	650
    119	137	1908	1324	1085	661	1557	1661	1828	1865	432	110	658	821	1740	145
    1594	222	4140	963	209	2782	180	2591	4390	244	4328	3748	4535	192	157	3817
    334	275	395	128	347	118	353	281	430	156	312	386	160	194	63	141
    146	1116	153	815	2212	2070	599	3018	2640	47	125	2292	165	2348	2694	184
    1704	2194	1753	146	2063	1668	1280	615	163	190	2269	1856	150	158	2250	2459";

    println!("Star 1: Min-Max Checksum");
    println!("Solution: {}", day2::calculate_corruption_checksum_star1(input));

    println!("Star 2: Division Checksum");
    println!("Solution: {}", day2::calculate_corruption_checksum_star2(input));

// Day 3    
    println!("Day 3: Spiral Memory");

    let input: u32 = 265149;

    println!("Star 1: Manhattan Distance");
    println!("Solution: {}", day3::grid_manhattan_distance(&input));

    println!("Star 2: Adjacent Sum");
    println!("Solution: {}", day3::grid_adjacent_sum(&input));

// Day 4
    println!("Day 4 - High-Entropy Passphrases");

    let input: &str = "kvvfl kvvfl olud wjqsqa olud frc
slhm rdfm yxb rsobyt rdfm
pib wzfr xyoakcu zoapeze rtdxt rikc jyeps wdyo hawr xyoakcu hawr
ismtq qwoi kzt ktgzoc gnxblp dzfayil ftfx asscba ionxi dzfayil qwoi
dzuhys kfekxe nvdhdtj hzusdy xzhehgc dhtvdnj oxwlvef
gxg qahl aaipx tkmckn hcsuhy jsudcmy kcefhpn kiasaj tkmckn
roan kqnztj edc zpjwb
yzc roc qrygby rsvts nyijgwr xnpqz
jqgj hhgtw tmychia whkm vvxoq tfbzpe ska ldjmvmo
nyeeg omn geyen ngyee rcjt rjuxh
qpq udci tnp fdfk kffd eyzvmg ufppf wfuodj toamfn tkze jzsb
rrcgxyp rbufd tfjmok vpyhej hcnz ftkojm
jnmomfc jnmomfc bkluz izn ovvm flsch bkluz
odisl hzwv hiasrhi hez ihihsra qpbmi ltwjj iknkwxf nbdtq gbo
gjtszl gjtszl fruo fruo
rdapv gaik cqboix sxnizhh uxmpali jdd usqnz advrp dze
flooz flooz qad tcrq yze bnoijff qpqu vup hyagwll
lnazok dze foi tqwjsk hpx qcql euzpj mwfrk
ilb fmviby ivybmf gtx xtg
rpauuu timere gyg wcolt ireetm safi
croe szwmq bbhd lciird vhcci pdax
hnc ykswt qqqmei goe bri wmyai hnc qpgqc pberqf bzs
hsnrb wdvh iezzrq iezzrq rdbmpta iezzrq kemnptg alkjnp wymmz
ngw don ddvyds nlhkoa aaf gptumum ugtpmmu
vmccke qbpag kvf kvf tgrfghb kvf bhpd sglgx
obomgk bkcgo yso ttft vbw ckl wjgk
fli qvw zhin dfpgfjb udsin nihz ovr tiewo
tgmzmph hauzieo jmg tdbtl lvfr qpaayq qapaqy ausioeu jun piygx
jkp guqrnx asdqmxf vmfvtqb tloqgyo ioix gajowri tmek ilc puhipb
uycn zxqm znft ayal znacus kvcyd ekv qqfpnh
fqghur xtbtdd ztjrylr bpuikb ziyk
rvakn uqbl ozitpdh uqbl dsej xehj
laxp haz jyd xnkrb ijldth woy xapl iqgg alpx gnupa ukptmmh
dyiy dyiy ihb qcyxr
wbwkd hdwu zvgkn hdwu wjc sakwhn zxujdo npllzp uyr uyr
fxczpmn cininu akcxs ggslxr riyxe ojisxe
ppbch sampq dnct afikor dnct edsqy pnzyzmc afikor
jnvygtn hijqjxl vsd jnvygtn nqcqv zns odq gkboxrv kolnq wrvd
mroq mroq flsbu flsbu
fyshor xvpaunj qmktlo xoce wkiyfu ukcl srndc ugwylwm ozcwdw mtqcste kpokr
cfh cxjvx cfh cfh uewshh
bpspbap bpspbap fquj mxmn bwls iirhvuk dmpkyt exrn mxmn
tvyvzk ezszod ntxr xtnr och
knfxhy kbnyl knfxhy xhkssx lxru uprh nkxpbx oodolxr tpvyf
nblmysu iwoffs upgof tyagwf aan vovji ajk ywzq oyfi sfulz
aushzkm lcaeki mkuzsah ynxvte rsntd refk pcm
mgguob gobmug dzenpty gmogbu
yvq eepof rgnree nerger fpb stfrln ernger
hrgkbl mzwvswk rsrsbk ieru holco pajvvn ztgsr qkyp fyeg owpcmoj
fowda gmsqdca yugj mcrroxv mqcbojd fjnqfji qdfsc jqs
qnc rvjfz vvxk sjd xrma ucdjvq sbw zydyt dfzww
ocajazv cozaajv tqunkla udwf ecnnmbz lsakqg bki njnda zsdu ccfqw rxpc
qqm qdfya qxyx qmq qfday uqnfttt
rnbirb iapor qet iapor hxkhz dfvzig pedl ybyb
mkgamxg xkniv meb hbzmxjn dhbj zhbxjmn hdjb
ilteux pyutyfx mau lrr bacak
sjjonmn dbbbgs crxyuu jztstgd ezb uiabyaa
tra fle ufzlvf nnaw kec hiwnnlj tei wld iyt syk hjdczb
qmd jtlud dgh dbanock fzp dsjgqru wwvo jwvxwgv xlemfij jcacd
rpkx oxesil snazcgx fly miiyc ikmtmp oefyyn egbw
ypfpeu wldnyd acchppb yqwcaw wldnyd turbz megci nbgxq xkc ypfpeu
iqqv iqqv neui iqqv
ypsxm icqyup zyetrwq nbisrv
viommi toszx dpueq eyy cunjou ffcjc jaeez djefra pxvkj liudlig yye
fhnacbg jghchh ghjhhc iue hwqmo
vbjw lpn cizba ltnsfpz tzoweml irewlc uzckhpd mszal obd
yeos utxkft hflxkfe fxczge qpgigkc ksgr vuumql vhlvv
xzmkv xzmkv krecdi klpem jsbu nwcmik emfzxf cjmpgnj
vtkjo pmiv zou gxo qdiyxsf hwyinjk jhkgf rjq
dyuoc ywiyvch irfgl ywiyvch fxb fxb
tuz onhr syu rqya abkaf bcfx mbknex juwoor zmksl
oheg spjorx ksdy vwtq fxz phvtazk tcze lrxg
hew lbup botaj ltr jpd
dxgc tzinkej gnz hxvvub adsqmc dxgc asgpp rqbdcra goy pmamdua bhiacva
xqv ygb kihxqz vyv pjcny vmyvsdv cgsi nfyx
tqga ssshrw ndq qlbvwh huyd pxbgj qbxk dkkbf jxy chsobw pph
hxl iwph iwph xnr otifm ljhre
zlgvpd kapxpoc dve rklk ogh hgnp rbrmc zzkz hhmcx aklmo
sar gfor nkf hek nkf aql shc aql
dtcrw kfjzcjx qyhi bldson whwdayo mqtgt xhqzp ttqmg
omspdml isze jdl nvwo qrkm wztfg ssfgyh dryj jhp unsmty
jxt cszylng ifht ixtuna azoi xutqlv jtx tjx
usgm azuayp fgkby ezpyq jqwl ezofj
tnhvil nrvg moyrpqs sldx qymoff megflxh pyhqwms xmdw
zomy zcquwnv lzx bvcna yods mjp dgsez
blklyf xokd gpit tiysj yrwfhm tofx
dtig vhdp omuj vhpd
fogwxim qvdwig emdiv jvhl euwbzkg xvxb hwmqo ujdmlp epmykj
sjxll sjxll pedvgb sjxll
drvay gtzhgtx yrt okz nqf
haxfazn pvkovwb pgu tgshw mxcjf pbe nwoymzc mxcjf pbe hydwy jradcr
prjsloa ahylvj okbsj qbdcdjt pmfo pagyoeg vkmhjzt khzmjvt opfm xfrji gyjqyel
lzypt jdbtrad ogr jdbtrad heink
rcoucuq gdxewa rcoucuq whlw zhhm rcoucuq azaqohe mzyli rdvaf
yuag ebcf yuag nsotg qqzuxr jfmao vyucw wmoye
qwvk xemm hgqrr wyxkpp tojndm xlvzypw jus bgnu bgnu nklfwhs
daqi knenmku ccm xkiuy vkexsbc kvvdagx umopitw yaocnx yoakqql mllmsp
mrxgl gywit mfopia ncnsvw vdxek axuiot rsejua nei prndudz mnu
egqn gaa qgen urs mix zbn rhn
ewharq aihy udkdaob kgrdd kgrdd kugbjtj fcef llqb pduxaq wcexmm
dwtiw nelq hppad algxgf gcc upou akm efnb mxmhrud
yxqaa ups okbhgt iet qns tqn rnjqxgp
npmhdm cgds ldexvr typi jyivoqk zkgq vfyxu xgfo
dkwnmr umm dkwnmr okpjw wqx jpztebl eqsib dkwnmr
dxbild wpbup evscivq dxbild dxbild geqp ojfbpl jshvqej
cxdntxs csfocjd pyy tuhws teb boyloz xfw scxh pxhonky
lteucke xrgwy hszgzu hnyrcvb
pfgsgwg dxzh fworek qbstod
usemcrf psczxu gcjtr brls
hjol efxczux bqdn gvrnpey yyoqse gbam ndzyj lbwb bhzn unsezg
bapw xifz blupk qqdk bofvqpp wnbuwyt rnwocu lzwgtt zucag pov
xkre lqvd juf lqvd xio xyg xyg
tzdao ztheib aymcf aorg iyawrch hetcxa iyawrch czdymc ccv
ucgl azlppu jvxqlj pest
dvwlw fuuy mnhmm okrp ualnqlm uyuznba fzyejk yaq crl ctprp
odfq knox mkbcku pxucmuf lpjpol phl
ixongh hfs ruorbd auy qyssl kykwcix aytsm rlj aytsm duq segpqhk
izufsk wedpzh podjkor eamo vqvev ifnz podjkor xrnuqe
twyfps bmdbgtu qye qkwjms
wlav htym vhsnu cocphsj mdsuq vhsnu jflgmrp
opajag itwjhfu purnnvk opajag
hpkopqp vnj aialpt lzrkzfs nwucez nwuezc
mcx hzcjxq zbxr dsx tpknx fva
rlvgm xrejsvn ghawxb efyos xty wdzdgh olahbtn rga efyos vhtm nsr
cni mbab qtgeiow ulttn rckc kmiaju jvbq emyvpew cdlxldn ulttn brhkprx
eykpffp rapik qki fhjgdyu tome ehjuy bibjk htxd vexvag
wrk dpxt gwkuiov gbkif ike gbkif pcd wpj toywyf qzsa aol
yqwzh uujn ujun ujnu
srs ralwxrz yxvvmgp sjhbhk waasid cqtxoxf whcladv jkmaq khjbsh dlavcwh
mdvsjh xaj etvxlsy fxgiy rgjesel rlegesj ptriz ebdyhkp kugxm dxv egljser
lhehwrs mqevb ygmv gri izop qgb ivm
loqqam alojlwg hgen hbyw qlwpun loqqam worgnwk kope
phozre todsknr todsknr ibj mvllsar
wuripy ruwlfbh wukbkey qhq iishw tvtvci xawvxc vxacwx hsiwi ogq
xryq vxwupqa zhqex aquxpwv bnvxrba dtbxki
yvvwh zvsm vqskhp vqskhp ggqqlw bpn wbuv
kqz tdy goqwge ygn jgd
szjjhdk zkpoo nxexz ebicc
wzuemcj oyd qupulju iaakzmt vzkvz
nppahov umm wpzev wxkgfxd owgekp bhhb bbhh dgviiw kdfgxwx wryb
bnc rhes lmbuhhy kwbefga bnc rtxnvz bnc
ani mggxf mcoixh zdd nai hbhzl mes bdpqr
mjn uinoty jjegvze bjgqg yhqsxbt coj obylb hddude xqi rhfbhha alood
cbjzj drmihy tfkrhsd nuhav hihzx bvblqpl tdd szmp gjgfv box
uumhdxd cmwgyf vepr rwqdkj exwk
hwvr ydvw bqefu kghes gvbhp awms iqsqes khgse
mrey jqfw fwvzhps komj dayvs fbui zmtd cofn mrey
dsjds fdpx irjj usndok qcctsvf fgk wvg txwxcl dxs llp zyilwtq
xmkelgk fdukc cye legkxkm wwly
enlny eynln cccku brkz dpof mwfoxcd yftmnqh wpebvyc
ggdn jnysl dsacffw ukj hdae cmzxku
uqhm gcachmn kxndfrl htmfis jfnajz fiqiypr kekho kekho ndcw ckrndub dejfna
keazuq ertql rauwl keazuq obmh rauwl ksrotm
jppp poigqhv repfsje grjk xwkyuh pkx ayzcj hoxzv
yhjw pcuyad icie icie icie hwcsuy wcd yihjh jnrxs
gaug ivvx ceb xujonak hbtfkeb ttciml cctoz
dggyyi dggyyi gqlyumf yasu fwdfa cbb nncn verhq
rhgcw gpcyct kiuhbg kiuhbg gpcyct jlmleo nhumm
wulxxu jyjek hclcp ogob viex wiqcupq
tthu nxgzpid kcnj mss ukapgkp nnc bxjocv qwxs oejwsif aywqtu brahkb
dtde bgvb smu vbbg zhlu
lyo nwjjmep ldbok wgxhto wwuh qfgjknk wnsl
lleyr onha hkwulbm jfg
bybjwd uoxvbh mvj iqfpnxs bybjwd zqtszp wvc lbazjr zkzenja cev
rbuyyr divtslq yuqmyt ajyveb smxsjb nlk tzqhq ims fewg wpjhr gqh
kpewfd beq klilis klisli eeezut
euqh hueq ldoo crqurv lvrwh tmaewp oodl
bqi lzrf jyhvxfh bqi jyhvxfh nbztd lwpdn cuzi
srjylou phavzjd wost uxkaq byh sluryoj
ihrdk bcegkpq nygrs qbcq wyjg dvzme pgzhjl vibg kvv
ijsx iedemek ktlz gtga tbal lbki gtga
vmiaxn kefig kefig vngxz
vrdmfvi qts vlvhq vlvhq dihmq
cfz dyrz zlw qnt vok fwvahg skshbqf hbwozdc ntana jdb uflp
rimbj bxemw sfps krtk umta vnk ewmbx nrlje ymrtqrz mxewb kjxunbt
egnuti ozat eltl ngueti
qtcwoxq rmaf qtcwoxq qtcwoxq
zws gcoa pydruw qsrk lrkybdf ugr wkrxoj nyvf vitwn
tmr hhd dojid zwrj bhsim righ keqlep flzunou
lwoquvy acjowxk tqudk oenvioh nyavyl
rgh dfhgyke iff cpxhuz hui koe iff hui dmukrei
bjiumig lcbmbgh vleipx sfawua rnf
gftfh qwb tfdroe xbno qhgofm vqfoe mux
ljdrr gyfggai iun nju xrucbis mhrcrh fukr obvuqc whlalfe xrucbis nju
nxjmjr egqwg arllu xqaahri lzc ivt uhsti
sqiepba rcmts kvesv nvp
tiksw tiksw rjni gbhvzm ctbq zuqfyvz
ibsnm kfka aoqigwo sqouih rxz
jmymq lxio adtmk umyu sxvzquq bporqnb heol fow
mepa eckq rqviawv dkqoei ifmngpp jiava rtklseu
yuycd jiufjci yuycd uowg yuycd udq izkicbr csxobh
nwu tfsjavb rruoxbn oepcov elxf rruoxbn rruoxbn azglwth jcjm ksqiqpv
dthfwip zqnwa zqnwa zqnwa
gso wruece ufl crgnlxv vllsm dpyfm wpa ctxko
wvpze seodz lpq lpq pmtp wsxs ffppx
yfxquj phvjn rtwieq rtwieq kgxztyu vbjvkc prqqd lyzmdo ojbrt ojbrt qiqjz
esaezr rpggiy jey kbzrhu uthus osr xxaiijd qfxlf auhzbx gkigoqw
yfhcj uvgck cds gjhhrg cmempgj yfhcj cjb
yxi voxvtuw unwg jqqm
igvjr ljz rus sru gbjtjt qfeg ztu zjl
leof ocxns hbkoysh hbkoysh leof
hab lyxmf yhh qeks fwhfxki xmbcak okqjii nfgzyg bhtfgdj lpmjn
mgognh tad herere lvwnzx ixwqs zphmuuc etdjz kczsf
mtej rlolsnn zbl uykek dpkan gmz etxtgj
mihuieo emjgbp jgks mihuieo iexrfw mjdnr bvp mcuzea xkbusvi
jvqpj bwt jvqpj bwt gxr
qpnd fpt tpor bibbpcg hmvguez wqc afl ckviua gpi
dntmcg jglm sxtnu sxtnu sxtnu
fzkbptw cbfwo ozvwov wbv gcdd izqo ovwzov lolewo xikqpw
nkxyxzd kpn datf fki werq mwidqx oiibor zizcjph
xvgyxym zor ijoy lvwsf fjuara idvvq rreit mqyyy ctio tzwqqhj rnpee
maqkfpk maqkfpk xukg sfdmnlg xjopvr xjopvr irf
liujcd vnlkouy dxkwc gto vhjvtw
swhqhj cas aupsd swhqhj cas bvbooii jquck dtdm
igh iqicicf ghi pcxt srcrjx gmf gyscphv
drplj drplj wopgpnk wytag wopgpnk
zexe ilcqoh qiefb txkuv lirfzv
ovvpn ovvpn uqeurqx uwzn hgmucj ovvpn sjxulms
rox silka irhsvym kutus otasof tdneav pcagds
mkja omu tyshbfq onp trxs lxa tftbv bnpl djhnc zdqfs muo
tjj rmmqas cbbkxs qio pikk ykyew gxlxt nhsyl ykyew
frcprg njrz oaxcmhc qben pedm ecvtga nzxwpb ior gaklot dpem
zyt kncau spoe qlchg sqys wkpbng yflju qlchg vkve bzadbpa
qtq pkaicl qtq mfkfqvr dnleiq brrjxsx uoyxh pkaicl yvmlug
firwy imtlp ywl qfa dqrbazz ztzb pcsbwhn zesmlag
ivey ivey mtvc mtvc
lhize acwf moa cdeoazd voktshy qmvqq jvmuvk ljfmq tsanygc
xreiqkc aawrovl pofcsg xreiqkc xreiqkc
cjbzvn ozds iniqu sdoz gqmki bablvll krs vjzcbn
izsod htkeqz entxn qtns prpcwu omfnmoy
kwfb tctzda aztctd tadtcz gyt wunbcub ydiwdin xxk
epnl ijcp giq ltfk zjcabve zfksmz epnl giq xxxbsom
ulyukpa mdjsbn dydko uhkdt qms aaaj hustlwu
zlsbu ohx jcwovf egf zlvpqgx qhejm wrywdmw
uhxqrzr mmu kjxcalj unuohiq rri yzngnb ikvlxry mfiym qbksdx
khqciz som yklmm jceb khqciz jspy jceb
ncwggv njvi nqox krtsn lnm
bgtqme xaxcoq qbtgme obqual vorfk baoqul lgrb
jli tsbb nlxjc pkwzmz dlxrj hmho gzguko ilj iyaasm
wlmw grkumg dynwtyo emxhhqr huluk slpqu uhqcmd absmr ufirmwr
pbs pcammxv dplfr tzvmav nccyy blvyq ffhnz bccutq
hgge ghge vxmvz hqxgjdg zab guo gheg
ylj bucoyoq udndc wpgyrbx ueh udndc gxdsdh hdoz wwgqlg
cjdeh gttyqe kdkm ltzd lfeozse quvjq mnwhokm kdv oojxm nxt
mfkzus knqxt saxkqww njx zumsfk sbmcyad cpt agvbuv
tukn vyco yobvsn bzgnn klrnzy kea thzk pxpwq ryfff nxzm
ylbm lxlz lybm lzxl
wgtxoij zad slgsi cvnxfg iomswwl vmx
hkm yinhnkj kmh kwkw kayknck chur styjif yknakck
rtfwhkq rtfwhkq zsf zsf
sldq zlntr ueegiw kajivqc ozcbm ceft snvugom pdyc elppeed nnqrp prwwf
lhk xjonc muc tudag tsafx mmivb dvrjbp qgrew
hnzer fbgqp aazta aazta lxaz lmgv aazta
victgxu victgxu mlpd ummrnbx cazjgnw isxcyp efy zfa cyusj
gyojxo onzq gyojxo uxufp awi ilhl wefwfxr gcjlt tmliynw uxufp pdcnxah
wjwachn xkuhfbp oky oky ybaeqkr rbuix yreoaw wepmye brvon aasb
kiidorw vxtxiqx wtqvbrv efdth isel qbom vcssyc vxtxiqx wtqvbrv riafzsw mqzsj
eurpjd vkhdamt tmfx czeoot hiz ykz lmixzq tfur jhzr
ipuftpj qbll sqkkdw fwncmiv bri oeeh lehd ioh wag
suima nanngc imrmc krq atxdo woy atxdo akev qlr aezco qlr
cfc efwbzck ozkmcxv moczkvx ccf
bnekky iakrk sask uwgnjp iyi rynev bdnas ldh kass
sicmw vvjbvv cap nsumc xgvrlm wsoo uoqdu psykckm
ugg mtr wnzhmmh tjxc ehwnji lwhu mdsckk yvmk enubrqo
grb oxmxz ohu ytetedv ssx apzlppg fdkamm sxofc jdt ynmu wyejok
umoep rbyqm eqfk twqnog cptbbi dragna ngqs ffb cexxnc rbyqm
utizi ormkel wvwur bdx ecelqbv xiccama aag glfvmj
znb rsuqoa uxo svc
obs lbifa cffi catpd
qkxwian ajlzjz wewduzp bbyv qmt fsr qgiu epinp ghmf
hatg bfgmb aght ghat
kuq inp dun cknbun wmwsu drlmmg kyxc bdl
bddybth swdbf jhi fva qpobio bjwm wjaztp jywi
mgckz vhveu zkemhp zdf xtiqqew mlx wazgd
umbjq pya lvvxf jeavij rhrxvew bwjqgpr piz
xaycpwo vjcuc qksc yuixhni sfbfb dydyaq gdfvb tggg xidphvf bpjdrl goskxym
agxfoip gguif wvo agxfoip ntkbaw fbyggy ooft zxih
nzvsu ffwq uxvfbl qrql olhmhom qhdltg ymwz krtndtx olhmhom nfsv krtndtx
qdp jqk ustz xjripzv mnk grnodk pjwdsj uug zqxjqj
mufrcox zunisfs ocvcge acamm xua vor bsde kxr vor kxr orccxx
ncycbp anvcxay bmm wndmeaw oso knmk mmb wamenwd kmkv ppdd
motdcn xzagzwu vuzt utffrn yuqxzrh uvzt ujttq
tauoqy coiy ybesz tauoqy wpmr trquyne ahxbj jzhems dsdy
aczq ypw pgmzz srfn quatjgf
cih ypapk bfxvr euvhkk gugru auhqui
vyf pssgfvy dnhvbfl xpacme dnhvbfl mzdv iynq hcqu
lbzvbu hhxiq hdfyiiz iyzihfd xhqih uzdqyxr
iapbdll vdr cprmrkk vdr dfjqse mlry flpqk vdr
grrfkq xcpxd grrfkq dxc bjpr prvwh swoc swoc
bopo chvwuhf qhd ieesl xey ieesl fnjcbe
kic fyq hsucnu agwyl pzzmd hqksh psw
mxf uau iti lcoz lpg zbu ocre wqlocmh mxf nidqj lcoz
bypmix ptzxgmf xmtzgpf hrvzzq
lbfw zwusma lbfw tuyyy
lrf uej unswvh obgsb npbl zajr kenea uej qnyjcu wzufim qpzkgya
qcrxj llyu kligt hlm ehwtbx dda lgsvhdt xewfcv uikn
nfzjx izqdbq mfbxs imiuc yqxb xlmvix izqdbq eflqfq wku omgtuu izqdbq
lasdwg hiy btzt eefd eyoep icn nnmhg otml rek luixac nyzgn
vekteds utsuxdx utsuxdx vekteds
feyov qrij zbebwg ijrq seplram wttkwm zewbgb kzuhuh
dmkgtv wohgqo ddtqmv zatahx mym hqowog tkmvdg
vhha wjrmuyx kqh vyyrj xzchbi ejsdq orlxg vyyrj dlrc
yetngqn zdtuqox hkarjei fqpsgh eaqwbg zsssog ghb gddqqzr hbg
obldb zsrhz zxp uxphnev mwnbc pfjft fms xwslk vjm fxy
nfij dbfykv ttq gyjgac igxuyqi gtiioqx ilhdex dbfykv uyp bdiwya gqf
pffzruz vogfosh dcs wje
pohhf fhpoh oon yyz
xxuam afwm qxl lnt syyr bwxhhf sozauq shlhfmz kwnn milav ochq
wefcqrt gejw cwerqtf fttf gjew
jfsvnmr osca epwtle pgfif sxom
exlfzmq nakp rgdnx rrcvth vhrrct aajjdrt ryyg dsozd jdqlqj pakn iruv
rmcvo txszcs xxhyxz hbsozk wshkocf rmcvo rcbnt
kitz yjgney yvkymef nauj hmllsgl kyhm kqr pzsu rcf pzsu qpte
cdinpx bfur mkj naz ihkheyr nohhoe
ylris xeqcgup wap bbfih tgfoj
ina gnlnm zyeqhij cudfuf ipufae bvkdzni aat teqsg cudfuf bjokrbl teqsg
aedx edax dnfwq qndwf
rdngdy jde wvgkhto bdvngf mdup eskuvg ezli opibo mppoc mdup zrasc
qcnc iaw grjfsxe gnf gnf
zbjm snznt zelswrk gkhlnx dqxqn qqxnd dmro
zisecvx ztezof uzbq otnrtj qsjzkwm ewvcp rlir bfghlq tgapdr qxmr
ipnqj opjf vabyoe wkwnd
wyf mfqxnrf apm snarf jqu aaghx pwecbv lvghayg
acncv jmmbwlg oiphlm ifuo cvt
pvmb egansnd zmh gcuzzci rrxpslv ubith
uoleptg xbouzn xbmg cfh cpn wpqi xbouzn xtxis sxzpns
rilybri kurbpq vfmjpck tjyogho hfyxad svfofx lfbbhxj khaerfs iqr
seaebgz wlmtkre qguv qguv wlmtkre
sgo edkxya zdqgwtt gxu nibuu rairqoq mzxli dci qsv
tsol mdhzqr rmaqnru ggvcq arbwkn hlkcnj ljkcuof
mmliphp ocup puoc eijjv
gmajqpb ijki ijki kvz
pmqss unhlpcj dlkll nuhlcjp expe tlurzmv nsy vlumtzr tgseozl
gkvaoni hsba hsba viuedv phyoclp fdq phyoclp febld nqfs
rxvdtw abn pntv qrqfzz slsvv abn lrxix mnu npot
ghlfjp woy xwkbmv bkahpkj jve cncvk jvdype fwgvoju yrkwjp gwfvln mvkv
kmluh mie bby fwer chsinb ojglqr nqk mie
yzmiu igkgca ybnsqja jpfejtp yjddy xsosxfi ingx qwuhb emrkwpx idqjmmm
btrllw mphm dkvo ewdl dchcul yah btrllw kmqi mtvgk wtb
hxsgard yuikc lykt tdee adprp gpougod klnzk mzsmlb
hdn znblw ifoblur bwzln dbv
smofpbs vjuyiro llk lfzesga tybu tybu
gffnpug xaup iqiyz fjkpnkz drrk fwyxw lwzfskz gslwpmv vjxylva tbkyo nib
evydmb nhwuiiu fkerq nkgbuyy uclrs ydjgglh xhotwbm riirgzt
bsub eavbt uvd dpzwyt rhn khrbptt xszckc djnfxju axofhat powmso nvdffrv
xtuykl fjz mbikc xpnx hmey fjz fjz
rkls nwdcsyx rkls rkls
tygml untequ ybdfumz nqffbq uipc sove hfnqj
ytecew vven koqn royynd qsn ksl qsn sdw
hknlw qwho whoq oqwh
lzmmtqu qvhyeo cnofuj utpwkjz gnirz yhhu aodbnd
zsr axw kwtzcv tydzo kwtzcv lkxsm
rbjtqe nihifd gvdxd bpxzy rxteky vgcgllv vbbua anygiup rqo
dpd wblfwp wblfwp wblfwp ygahc tqjbaq
gsw gsw pacgj xmrcz zmxhmch xmrcz
pdq rhe xqmq lgpkhg fyffrot ovnqh wle
tbjavke ypzzrj jizx gdxoh icjsat otfh fmygumv
snch nxlgjgp jeyn sxoqfj jtage jtage iuice
rtb coefuj grwg grwg rtb krhqnma vfhgbr
vhegtl btorwxg szcev kbvkx itsk nlzpbed
hiukrf ilzkm yllhh xsgwkdp zyy kjbv
rfcg tdorci zcj wzftlv rfcg rfcg
lgbc lzizat vsno pau nvv vsno bbr lzizat qhtb gwp
sfwnio tcugjk bsfsz ykyfwg ibkap fsrvy mygk kzunawx zyhyh
mpavlh qps bylh lttjkz rqabgk vewb bwev tlzkjt gzrbxga ktmso prpkj
gpf ims ynh ffrs vpa iemp gofh cgbauje
secys qks mcnfhwh drog kqs pajy zoltkw lfihnb myb ioxptu
ytq nrta ouk ajqblf yuwwcd zdy blyoxbw dakk nvgi bzrhzaa
nkoych sufiia xkdvw crtldee zycl qblab egqhr qblab
nllno muxaf vds qjnitmw zkpj wskyhft kmqct xamuzpw qcai cdjtbt kaxv
qzdytpe osr fuw osr qzdytpe whperd rydwdcl knoa
zkdznhd peh duoygr zamrgl irnvj otpe pltpq jdkecg
byzgw rece iigdug ehif tpgje
ccnn foqdran gbctca tefdjxh ntcr rjciii xip xlss crl wvvhzqm twyohf
dqyii milqqc qjgkojp qjgkojp ryde
tdkyj tbrcud tsba vqtmb cjwxnf
hqhmq wemvrce nagig pwnw nagig epg nagig vlsi
tqgvw luoplw hccti npjm rytdruq cylrsun rytdruq vjsbjl rytdruq ppti
itgt tuwc itgt rvp itgt tigns eipl ksmru
pdw wdhtkn nbdbpn wff zhuuipg rvemv qxr
qgkwdq cjilayh ymeks mrpuzai dwgs stfstgz ucvqhb yout oiq
vpxik ypfr qytimvu qms oxbmw ppyfx
fwwidn gdhd pyuexk snsz iwndfw
lfcb sllxjna lfcb hpzahfg mmvgaa svny jhuzd
unyg gicmzd fwc spkciy toyq wjupckd vzzx iuqgka ytqycb pxsufj
goj tnrcml eyizngj txa xrkiw zvu igduz
wek xrrlkna clyof rrlnxak
cjm rmyuku vjom gtf
buk cfae awstd dywgqp hxo wcxvf laihqw xdqfes wdbh qceh uzlwj
sudguo dxwplto rlebdh bkamu dxwplto
crwkyxm yuz kjtdhom crwkyxm
trhc sduorxr aizfryh rsudxor gbyc
pczkyl bptp qnn nxmpwsx udrg hhlb rubtrmx twzodlp xygnht
jmqct cden yfajtkz fevcw sxonbxz sxonbxz qkzkm hhngr fbv
sdsnm mwvicr wypfi cty ndbowr woiz mrauwzd qlno mwvicr
vteyo fng lvr lxytn txpj milg
wjx ahtmgo cgwcaj kaxae fhlvlqf
ezj eetqhzu upwda iiefwlk vyvby
imalvy yeghqe jwcu mvrod cwju
bxnmsa yhfu npsdar tsbri hfuy sirbt oofxmy
fkndt elbjtn vepqtxt elvpf fpelv bzkgag qttexpv prblwb
rmq iqs yvprnyy iezqrzm wlqsrr
yviovq lekxghj oey qwhzj lxknxw qiyovv ksnt jptz
tyrg cifxt hugqf tyrg ffuiv jmax qyw fozfosq ffuiv
nmg rsl jpzazd qbtlf yxqtsj czwmdfd bamge lbjdof uqy jssc
cbx boozjip pwgvzlq rjz kxy kxy hszacok fvsq jhnir cnsba gafz
sbcuxb wfur nnnfqjj fdwg huhe sbcuxb
icwk qelbxs uevp qped zsnhh wpuok wddxsln ftnzupr ruxol cgxjb jbhh
izcp htykj xxmndoq amnspe htykj
vverol oixwlny vqd tvfzu henc gnyrwr
ytxio etytsx choynep zqapo hfjit
lkvgr oyzfa taiqr jok djatvy ckif tmdw oyzfa zroy
jlgpyp kkqysg oqjki hjohoug hbhta muilz zft
sumfyu wftcu bwwdcy lezimwa qwvxv zwh mqyv bmfot aii torcol rnt
tpdj xrw ccsbnh fhptv fwkxjfm dmqaokd bjci
zxi vmf vmf dpyg
sfzxysw lcms bkojtv bkojtv
opywo qll ipkitr mtwp tudrr svhyp huz bxsdpn xomfy
gkod luo qrosbp orbd rpsjzyd rlh gdok tze
nusiuq nusiuq zeys ahufexc
veno jntg avtmtdn qojxru zegdcql odfcetz pgehau
uqun vigjm ykac ozlelj danmji bibugox
rpuozh ajwru rbvuevv uhzsq
iawoe tyb aewio ymf byt inijv ctu fcys micsgzl pbby alt
gktyxp ris mqpfm bkqsfl nrg idbbcxg jhcf
qibt invvv qibt luitx rnm eby hrfbmwl wnap sgkzvb qlwc hrfbmwl
jwkv qecsjbw lycgldd wjvk tjcp dycldgl pzrvr zrlcf kji
nzsrmiq nmhse ilivrk kqv
besmyzi imkgpt iekbjax abxeijk uvzs wwv
jdocl uki ltswp tjkljc ymce iuepze qygqxzs tei lkry
hhyfy gvzd mqksxlq czn afe mesnag eep frwgekg mqksxlq phpy
ehg connnza ekt ddgokw
mpbsoms uzhzl xevww ztt uzhzl
lftybr firc awsud dsxdkk ltf ipjv dtx lcymth
vkcpb gxtxq yioeq fexj xxgqt
srvca fslnnvf nfmkpvt egw wemumq jie vznf dzsjw cukf kcvyir
yxjkl lyjkx jyxlk kgc xtz
tpoe xzov csp leleoqo noyre tdhf cyib sjgtdx raehdw nmcxp
qvt uhznqe bpvos vtq ddlebtd tqv
xlw utsxs gpia rvlvnts elkxr dddihy tnrslvv ibf wlx bxg
cwqnnrt rkkqyf dye yde fzl pthanj
boc rqjenpp xjqte jteqx pvoofc pidqe ruoucy gvnro ognrv
qhalb gnazwc fhl iuti
clnbjfo nnfs nnfs heymvr oarew oarew nxu
lwtrotg hiaxwj ymzbly nvhzjhj zlsaheg nvhzjhj ymzbly
rrvi tsjp tsjp tsjp killji
rpx hiclj cmwq ibhj nfd
pvwymn iebkd xmpw vuhhkap ksw zigzy mzzyyxy rmuh iwwhea cglfq
rlwelgy sffml jin qsdzro xlsty mgqzuu etxjuo emzd jgnoyq tkjuy vfvb
tkctdj hhkuc viskmy obw
zvjkuj akeky ikj jqd hfhzbwe bkc
btev nrdo hcyiuph stf qharfg vpmel mpfz nvs ytgbbc
ieepn ndueuw svmdr tcvumw mceyrn mrjwhyl tbdj mgrgvz
uxrs ckyi xpmqm czzrkl cjp
nlliwd wrqkrkz yjmng nlliwd zirde hcjjn wco ysf mgl
dxti lcahe ommare izlwf ramsfb nzgfvo ijvm fwymrdu bndq
isxy jpvuzu tdduyhw dixp cfa fkzbteg ytoi kepk ysf yqcpi
qmeprfj soqo ncgeor cqsuuj grzy wogxy vyblnbg slvtry vdols kka
ltykfp gtzl olrp gxend vapee deq
emywfbn dbfiut rkt wvwe dbfiut bwffhea yuzcxv gogpicp wvwe
vqvmrp ofbk dlfabd jwllzxk obx vqpwjj umvng tqwis fstxy fstxy
miha zgvyux rmraszo xwf
kjaagk btm kjaagk wkewjrg kjaagk
lbmli aizs omrdr gzktnx asiz ptanzpa xlo ljre ckyb wob
svz dlk rijagg avxmg fkzwhk uro gegm
dzplum temdw jqnm tvxcww bmg tftttpp deuw comxey xfimzjx caluczi nqn
uwvhxa ztkd nlsdyt vihl julkwwv uzch dwakhs
wkhuihh ycrc cxff vzcfhpp uegfd gaok kcnvz lhzogq lwa tyrypvu
idp zmrrzp zmrrzp nktp xsnx rjsxn
eybrnib ivgntl vaxsbpi eybrnib
nzvnq xvbfa pbhwwh ylju runvsj imlx vztesn
nfdohd nfdohd gtevnky pivjyct ihvd fzcsrq lko fmqk
kwpkks ecikxu bcxswlt qvrxm sbcqmh
kdjrmj piuh kdjrmj vnaf gyedkg vptxgm xezssxx zsg qjzpo zsg
oqo sley aqx qmpqb fgmylbj egd zivj kepxizv kuakyn lunbnd
hmcf hmcf xlhgc hmcf cdlm buofnx
onjcj yluonz kzmk phqo phqo phqo
ohaafy efl bnkkjww wwjnyoj dxeaig ywnjjwo slk hrbebw ohlyju elf
msohiqz aunk njki bfktdgi htmyrj mgx
numlzrl rmnlulz glb ltt fhbajz gqxpu
gko hco oai ryq xwy sdqosft spjkiu cxfhg ycwpglh noy rah
btzpjem brpk vqr atxu rhlh rqv jmg fvyus
phmxxgj ejx xje qtk hsb kqt npwj gqt
hujyjp nwmsd ant zipuya lrkahww uwqal vzlo qmbo twkjkse ufivi
zfbnyz fwvh xrnrw usn zin daq iwjzj
yykyg iwypfy hehqnl cjvk cevdrec
gui muuto wsta glqmx gfo rdmbv mxwz gffzt eejpw gion
lpng nduid iqbpu nduid knrqd
xwxn oefpckv gjaua ugaaj gjuaa
qxk aeql trqdmqc crzlinj crzlinj trqdmqc rijcne ewyf
rfv qmbe fvr bmeq
upqyfw lowzq wpen upqyfw gfskbil sljuzh wpen
bdcara qyhx rtaez qyq gbyr
evzls qxtxq clzd svbgqi zxlzgss vtrre fko eebo qjyl
zaapeo kpwhz tygknau nyd pch trp xqe
ypzcafg rnqmbh qtteg sncu ssojhhm zonfym thir xmgheb wqj gpjg ssojhhm
wvcwyn xrf muozyya lasdp xpjgu kpqv zkiihiv ifje cbdlavg xbied hfnaa
qqqb rettz rycukl ihpkhh
dnxzxqv znb znb fbxj azxtezb xvxa
peqkd xlzqkov esgnw ucku hrwpfxd xtd vnig vlmfp ajte qswr kqoj
dpwy oavzkk dwyp ehij upqxgii pydw
amfc hfv xmqa nqvn cal rqmcq oej amqx cla ntxj
hqhhe qkbhwli wmhlcq xaczs peywuo
vcr xfv xfv kymo qpszwzo xfv
nmrbur tswo xbo ljlrzo bmhpgc pev zovkznz lok wbbhtkk
tojj lxqgr rhjavrm ndsdup gdbjwaq cqpnl wfaxivl rfry ryfr udspnd
beffod sknlph amb feobdf
mldgn jxovw yuawcvz kzgzwht rxqhzev fsdnvu vluuo eycoh cugf qjugo
tlnd qcxj ker fdir cgkpo nrqhyq raef uqadf iahy rxx
mhvisju lhmdbs tcxied xeidtc ujry cditex gvqpqm
cgc jazrp crgnna uvuokl uvuokl uoiwl sknmc sknmc
rvbu czwpdit vmlihg spz lfaxxev zslfuto oog dvoksub";

    println!("Star 1: Passphrase Validity");
    println!("Solution: {}", day4::count_valid_passphrases_star1(&input));

    println!("Star 2: Additional Permutations");
    println!("Solution: {}", day4::count_valid_passphrases_star2(&input));

// Day 5

    println!("Day 5 - A Maze of Twisty Trampolines, All Alike");

    let input: &str = "2
0
-1
2
-2
-3
1
-2
0
-8
-1
-7
-8
-11
-7
-8
-7
2
-4
-4
-6
-9
0
-20
-7
-16
-10
-25
-3
-17
0
2
0
-29
-25
-8
-2
-19
0
-38
-29
-5
-24
2
-1
-30
0
-28
-43
-22
1
-12
-23
-23
-41
-12
-51
-56
-1
-57
-29
-32
-29
-55
1
-28
-29
-49
-56
-13
-67
-40
-10
-44
-32
-37
-49
-37
-72
-54
-22
-24
-31
-26
-45
-45
-45
-23
-41
-85
-2
-90
-31
-47
-81
-40
-43
-51
-61
-41
-64
-60
-5
-29
-18
-104
-70
-68
-53
-17
-58
-107
1
-39
-36
-107
-99
-60
-13
-71
-117
-91
-117
-36
-102
-29
-23
1
-90
-69
-10
-28
-94
-92
-13
-121
-57
-16
-27
-77
-81
-45
-79
-49
-6
-14
0
-122
-87
-75
-67
-43
-113
-149
-144
-48
-6
-104
-155
-136
-85
-136
-157
-149
-25
-18
-61
-67
-34
-108
-129
-102
-9
-145
-95
-21
-144
-21
-92
-135
-121
-67
-64
0
-74
-175
-105
-120
-169
-35
-41
-92
-51
-76
-63
-184
-163
-189
-43
-58
-84
-27
-147
-147
-54
-26
-106
-83
-58
-55
-124
-145
-76
-81
-97
-88
-105
-76
0
-84
-50
-106
-105
-153
-75
-15
-40
-156
-125
-109
-17
-147
-180
-156
-94
-201
-49
-187
-13
-9
-196
-108
-120
-94
-27
-96
-68
-128
-67
-181
0
-231
-125
-33
-241
-194
-255
-41
-7
-146
-135
-70
-157
-232
-21
-153
-182
-130
-263
-222
-137
-176
-192
-84
-91
-154
-83
-69
-94
-90
-112
-44
-52
-193
-240
-5
-140
-156
-185
-176
-221
-180
-60
-71
-155
-238
-28
-165
-265
-22
-176
-89
-174
-289
-52
-110
-84
-54
-53
-250
-189
-109
-227
-309
-93
-173
-171
-168
-278
-196
-103
-130
-49
-321
-105
-1
-219
0
-81
-62
-55
-250
-257
-223
-22
-67
-55
-12
-106
-169
-243
-17
-308
-181
-310
-220
-232
-83
-12
-126
-265
-84
-273
-264
-253
-289
-119
-58
-195
-184
-340
-230
-5
-283
-68
-137
-41
-31
-210
-97
-354
-79
-267
-340
-235
-340
-82
-166
-225
-95
-68
-130
-14
-218
-239
-115
-40
-10
-20
-381
-58
-10
-328
-323
-133
-139
-224
-96
-158
-219
-112
-360
-138
-123
-303
-58
-209
-309
-91
-276
-5
-352
-66
-301
-379
-222
-88
-110
-374
-266
-334
-382
-189
-106
-260
-322
-269
-75
-117
-36
-412
-62
-285
-291
-188
-17
-158
-415
-285
-235
-223
-208
-30
-273
-276
-239
-93
0
-250
-251
-421
-368
-253
-253
-419
-364
-343
-188
-352
-147
-401
-55
-449
-171
-382
-36
-250
-306
-72
-278
-229
-69
-145
-16
-455
-474
-211
-183
-265
-306
-425
-354
-6
-256
-397
-252
-409
-126
-383
-325
-409
-431
0
-306
-52
-219
-172
-346
-444
-84
-56
-402
-112
-62
-172
-358
-329
-221
-371
-174
-388
-9
-168
-56
-109
-511
-161
-282
-344
-437
-292
-423
-308
-478
-175
-169
-468
-54
-439
-231
-357
-500
-414
-101
-53
-71
-192
-166
-517
-296
-249
-153
-9
-252
-130
-307
-240
-312
-242
-377
-48
-57
-6
-96
0
-124
-463
-68
-309
-487
-448
-172
-553
-165
-399
-223
-45
-190
-552
-209
-238
-458
-199
-154
-212
-53
-347
-316
-419
-363
-407
-435
-150
-203
-525
-159
-214
-216
-9
-302
-1
-158
-309
-33
-168
-539
-461
-171
-274
-68
-126
-372
-316
-160
-212
-261
-570
-386
-49
-494
-428
-458
-410
-419
-380
-25
-26
-36
-328
-303
-412
-169
-140
-359
-112
-198
-517
-180
-459
-550
-529
-413
-219
-223
-518
-584
-253
-552
-287
-280
-129
-187
-531
-583
-48
-222
-598
-590
-399
-488
-457
-290
-259
-624
-504
-336
-594
-435
-328
-656
-413
-195
-33
-574
-289
-60
-180
-640
-517
-26
-359
-157
-81
-503
-530
-21
-274
-84
-619
-68
-568
-483
-229
-499
-516
-406
-511
-290
-125
-90
-486
-655
-342
-347
-311
-121
-83
-53
-404
-486
-491
-155
-208
-90
-394
-379
-451
-375
-404
2
-70
-688
-516
-654
-453
-481
-111
-144
-606
-321
-113
-513
-457
-337
-638
-5
-72
-496
-136
-59
-528
-337
-445
-295
-488
-227
-226
-115
-120
-121
-103
-320
-203
-700
-257
-385
-76
-334
-254
-395
-673
-118
-202
-178
-415
-347
-491
-266
-310
-102
-266
-48
-319
-56
-433
-185
-15
-766
-694
-714
-720
-707
-491
-541
-69
-27
-21
-726
-578
-287
-544
-432
-351
-155
-614
-220
-417
-206
-496
-496
-487
-245
-634
-95
-705
-273
-342
-425
-488
-97
-710
-324
-464
-169
-298
-52
-288
-353
-71
-41
-226
-795
-46
-224
-712
-301
-559
-371
-122
-547
-446
-580
-583
-164
-812
-234
-383
-93
-112
-541
-183
-199
-171
-826
-665
-86
-300
-314
-382
-586
-471
-838
-25
-359
-352
-205
-418
-621
-387
-582
-563
-520
-649
-476
-202
-509
-121
-545
-479
-307
-614
-476
-552
-134
-198
-198
-482
-745
-680
-443
-362
-39
-353
-829
-727
-563
-66
-306
-224
-145
-182
-820
-102
-25
-307
-203
-469
-438
-74
-211
-394
-723
-406
-671
-356
-726
-792
-288
-23
-398
-459
-221
-133
-269
-661
-531
-502
-737
-666
-359
-375
-834
-629
-767
-882
-358
-865
-875
-740
-816
-661
-378
-354
-596
-729
-764
-262
-802
-374
-293
-661
-435
-168
-928
-301
-823
-470
-519
-692
-589
-939
-855
-699
-585
-632
-831
-575
-357
-871
-844
-667
-366
-772
-766
-594
-660
-302
-894
-124
-518
-216
-498
-287
-6
-334
-892
-391
-419
-236
-508
-758
-823
-824
-701
-766
-317
-186
-375
-421
-246
-362
-4
-678
-202
-506
-801
-665
-689
-547
-831
-391
-174
-475
-587
-747
-870
-164
-975
-336
-564
-248
-340
-242
-641
-510
-827
-634
-973
-196
-83
-798
-393
-414
-617
-919
-21
-129
-831
-502
-139
-858
-967
-1020
-19
-622
-878
-63
-799
-171
-277
-395
-166
-793
-745
-752
-228
-287
-997
-720
-864
-10
-578
-479
-488
-265
-1032
-909
-157
-633
-773
-1009
-61
-988
-896
-995
-792
-647
-305
-294";

    println!("Star 1: Jump instructions");
    println!("Solution: {}", day5::count_jumps_to_exit_star1(&input));

    println!("Star 2: Weirder jump instructions");
    println!("Solution: {}", day5::count_jumps_to_exit_star2(&input));

}
