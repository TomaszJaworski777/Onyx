pub struct ZobristKey {
    pub key: u64
}

impl ZobristKey{
    pub const NULL : ZobristKey = ZobristKey {
        key: 0
    };

    #[inline]
    pub fn update_piece_hash( &mut self, piece_index: usize, piece_color: usize, square_index: usize ){
        self.key ^= ZobristKey::SEEDS[ (piece_index - 1 + piece_color * 6) * 64 + square_index ];
    }

    #[inline]
    pub fn update_side_to_move_hash( &mut self ){
        self.key ^= ZobristKey::SEEDS[768];
    }

    pub fn update_castle_rights_hash( &mut self, castle_right: usize ){
        self.key ^= ZobristKey::SEEDS[769 + castle_right];
    }

    pub fn update_en_passant_hash( &mut self, square_index: u8 ){
        self.key ^= ZobristKey::SEEDS[773 + (square_index as usize) % 8];
    }

    pub const SEEDS : [u64; 781] = [
        6010607256382380006,
        386869187810051925,
        6942428122597393202,
        6279097271147704030,
        923295755687546747,
        2456028627973832580,
        1185049707845698375,
        2919135081950136457,
        7108059193585704106,
        6875260734772903661,
        5529384020042190545,
        5755921163823301543,
        8012196333246168770,
        6783710900469244172,
        5830071269551429892,
        1249494859988838658,
        6509108842230180222,
        7245894554766110339,
        6883096413978922832,
        9012248102179627431,
        8027070891353545387,
        8424375122720454012,
        8988003994089085929,
        5836826320809054550,
        2856125096126667904,
        5996961422774616450,
        2405620207059951091,
        8381839291733981068,
        3608829740989985740,
        6838849161698947255,
        6250320899433452684,
        4861983845157764271,
        3979746727128238998,
        7133072087162922887,
        2771457403129738731,
        5519848474801621651,
        3261326338191656069,
        8498341279445826187,
        8065347819450948604,
        6707954132216989668,
        29711481083246791,
        1526678906188282014,
        3104717776576893292,
        4042383814264601344,
        1473565573976541301,
        3739513578613105384,
        518591888542563073,
        8305764628238146394,
        8136519014862349763,
        7223199137396218100,
        5222924393111496316,
        6214732934542515378,
        4294552300776240144,
        363893188667078686,
        6216950098710642762,
        9125652594586937969,
        5653073031261379490,
        1046178841769328717,
        369614369086923039,
        7163642849173728031,
        1189012829239454607,
        8302823335425735376,
        1054650222736199402,
        8455398289508922864,
        2460154648277742507,
        2841106212426827419,
        5776520229169727016,
        8880019527127353922,
        806159662961766844,
        944490230052103759,
        6483754181317801023,
        4023611430482685535,
        8599357648115678627,
        7315740288653758502,
        6890178939705206853,
        5761163253433868236,
        215743686798364111,
        2567594525640542665,
        6204358145395626815,
        6230683106748455875,
        7014317022481308320,
        7626290351680904001,
        3786479886725268626,
        3279334212521414279,
        2302629781599843766,
        7646640675822179424,
        7901823121132092415,
        9095089245240142584,
        2092271162194201214,
        4117205614285836338,
        3682038935647309475,
        6881533684279014317,
        5829772442493426129,
        470217603122943292,
        383752253843933378,
        6988287956840650615,
        6457406537222271533,
        8290801541932262971,
        4776392529892151506,
        282871622806468789,
        627696959107841969,
        5480056865482622290,
        1429695648339312861,
        611754579400836980,
        996010532865232580,
        6238142675095184159,
        7827708125122264746,
        8061478210242447010,
        7380762980230752382,
        7242986029704506586,
        7969473703690170921,
        2317974000080453716,
        8535360882519309855,
        8894234287720833281,
        2014466810094994440,
        2456681485041348265,
        3950600203087529172,
        5743294126706393344,
        4416026813798610324,
        1177494710484229520,
        2939145971495320497,
        3912781955053895078,
        3534835051597560196,
        8703653898258310197,
        8809610978041776741,
        3287438603844382613,
        2154610997994830552,
        1413564670172202005,
        2296040038943884986,
        1277913282945553552,
        9185404238449355677,
        6342752589795729284,
        2101211062520470435,
        7243658148973908194,
        3339769928935119377,
        1925578568403724835,
        3841605284520615747,
        6519805401188566458,
        3681131804255683702,
        7441561072724123533,
        858368821450394955,
        667867024220676241,
        4362821092027917135,
        8986807205914617842,
        2543974602811069919,
        5995356892386377946,
        5013718270575889517,
        6234845775553307030,
        4216989678556107308,
        2948020678468936402,
        6997115525219889373,
        3257944587352931594,
        23617597768653918,
        5139059285073982651,
        2750858916201116047,
        7799395464957163805,
        5508843737851399346,
        232676144034988560,
        8913153620184808428,
        7429673810084632721,
        4556661334685702241,
        1326194251475406550,
        2998096421233098291,
        5298785188693228308,
        4034634305268153873,
        2416113791489519180,
        3544093613181918989,
        5744215754667690880,
        8261947220934626522,
        41653052596412710,
        6783701695492384240,
        1152449004204022779,
        4910115461950966350,
        4403522332638788770,
        9077145637818633251,
        2891715776319611718,
        3137357312271296819,
        3881449245726122190,
        3507290794643910825,
        6041433280043978019,
        6122894023149137151,
        6297240677024832800,
        1587719445491832143,
        1483204168818402765,
        1239110319952350384,
        7850309278641755357,
        6235828891129217240,
        1841492453728819975,
        2187901228659713883,
        4166887609490607190,
        6056434230394364973,
        3901381936410428650,
        1046592267366794047,
        8423487439043041661,
        4345373283440952718,
        1740965015394458914,
        4033648112314206230,
        5264991926450998843,
        1581063581304090528,
        6175793957192662259,
        8708226978422429233,
        6014653915005757322,
        719024066749754112,
        927050880556035924,
        1704706112373688018,
        1468864067991406122,
        8460797973687851535,
        1953970869460752226,
        6705893845996560065,
        6989330642002692041,
        6261807344014367982,
        2160019962098147526,
        2466827353309452066,
        4027796150219710108,
        167968740790043,
        4320665529759980544,
        5833630961012834207,
        2545182364984421378,
        1253739077731622733,
        3969648929981666098,
        3549555176103005898,
        7516560971560498784,
        618586394320132047,
        5620207729187933286,
        8524021150547643956,
        7917276138650903425,
        4573939520098945321,
        2409757404293773043,
        4961481161856316806,
        2402525920100237499,
        25847398273277431,
        7095401483046010049,
        2882426641756846380,
        4110978984840164470,
        1466213608507481836,
        6824133989534910169,
        6862290789314005945,
        2784930247437076582,
        3192120180590817850,
        7756445500175455974,
        6595839911697654274,
        5680559785692794126,
        6971162432404845890,
        4497435615873715518,
        2308937495890952575,
        3196823981438359641,
        5852874779755277392,
        6190473357455304883,
        3455888541065007326,
        3483226504874571573,
        6132120154115880085,
        7779410565022113339,
        8809728939064140090,
        2333577142108996858,
        6279750320922795356,
        4030797680387586908,
        8329920589749297645,
        353880579286129302,
        8712026258449712899,
        7515221208740001996,
        6346797385559614083,
        1433165584054624283,
        9045585814142300456,
        1820990401093059941,
        3805908565673580650,
        1478000815693119087,
        1534512345299450159,
        4538352129652111744,
        5449505478935102265,
        3226692200904508877,
        2014592490935273042,
        8935944136849783937,
        514838860473205518,
        4507568500031964299,
        5889419892271561732,
        8115058973337695157,
        3961700191090233220,
        1874300076741772681,
        444939071630183086,
        5682769610239267721,
        7645792591416798310,
        9218393096416096162,
        5221239266549819702,
        7632601998942653427,
        5640445578242201891,
        9017541975207868745,
        1055660176880144364,
        5743036552484595597,
        7629456490044710423,
        5629110244028259698,
        7925820469136268244,
        3952942460192045338,
        5512902142138882552,
        6200242659084933732,
        6884419997987072335,
        4272796190825114760,
        3716178818339058899,
        8305330917892773647,
        701484922487559568,
        8835202214365294755,
        4843851705520703299,
        8480375565180829831,
        1152627995371565799,
        7341248023254985527,
        8429304976237208008,
        384007104443775973,
        5229723253347164231,
        6897263570174223621,
        6985016398867928768,
        5585631130574435766,
        6115356238394586895,
        1910152551740672453,
        586721165007976149,
        4820271046239619539,
        1705510850952240710,
        4948806476728207297,
        6211637988601124254,
        948652441013016237,
        6889909794463591218,
        1706482634429872466,
        4186588663865071634,
        6602981940167347729,
        5364253465427141534,
        7807692795479614776,
        9176460695541669868,
        2538361300274470839,
        1475151770416805659,
        530627791866198667,
        3026215433270839217,
        443181377872945615,
        3981710099374172986,
        1468719606000270312,
        498574235505090678,
        6856576979889549006,
        8164985061694791401,
        3832341504575886714,
        4652799834332269078,
        7953557987126758872,
        4444077209637850632,
        3751303575503951362,
        6619177196021091986,
        3475591315449490190,
        2904669589922500957,
        8316893853343955498,
        7675870456608567628,
        9186609085561209516,
        2479858545880628900,
        1952582934005692847,
        1557292598279636045,
        938465052396167329,
        2794748311859476812,
        4884300323351044178,
        2974110148984701160,
        8136146688807733264,
        7188241645661289695,
        7799459413344754977,
        4836886981305594085,
        7776696905529192587,
        436976729374143791,
        81604500412702549,
        721811694606159989,
        7794098615530930979,
        3944574033048066043,
        1582117935676176294,
        6922127200043652892,
        1532051437982528140,
        8758020246150539276,
        6587391630814057837,
        5333429405109840453,
        1148321085261727482,
        2470457334195056009,
        2328340779566370310,
        8575772504512564834,
        2688204291055743845,
        7204027725029820842,
        9060415924374586021,
        3535103353714408968,
        4827108259725641864,
        2990369177223769508,
        1324075578591620425,
        6117638919896265157,
        4125058505738391346,
        5132604354476282148,
        3018453955840694007,
        2920393873799937447,
        6877295033070548573,
        5421339323346889994,
        5868256947721829714,
        4235025809802810850,
        1660724706592007427,
        2842417978013580745,
        914791546010535478,
        7842493478418027357,
        6515638038651105953,
        2598647398301283451,
        2067712262897397797,
        6106238159925475456,
        7534017883868378667,
        2368021676530812073,
        5101982348675734645,
        3615052962436574385,
        8498423236881280487,
        1311307530021855919,
        8013036935738712893,
        5178237892172036210,
        1219727947394776961,
        2210450442509240296,
        3824441938756571631,
        8325907908611617272,
        889582836012716925,
        456452019182909909,
        4035855961832161125,
        650903204658111132,
        5347168898198673125,
        6566045912606589241,
        1084081868365953009,
        1680901598410471700,
        2919386187041515425,
        111621657528262191,
        6645589141636684640,
        7754754342985121543,
        1756659553086896595,
        9136281958781501577,
        8475175850354128240,
        9103763297002104146,
        3350594040375870305,
        1930053665977684909,
        1147230433938090004,
        1241217005944913950,
        3834509054963684428,
        1483212002759056474,
        1315737711899952490,
        290922848610407420,
        3749955666434383622,
        1093622274631823702,
        6619344404813236094,
        7969316761857750465,
        56802800034426840,
        3550268994349640413,
        1874846073961774356,
        6163236959267636115,
        1836114857853210742,
        64532993065763991,
        8708707678689499913,
        8133183856387693316,
        5547500041146367526,
        1818242266132347649,
        8752272609408533443,
        6541488736951903891,
        932719834861999249,
        2110271368472934659,
        999607425767796274,
        3407079453568420094,
        7453915111005535686,
        6456994442476535228,
        3981227457505172770,
        4835821164883522357,
        8179773863106848279,
        4460415957737479422,
        1698577867812121570,
        319002993452676544,
        3243596365210347978,
        711596887373698435,
        854914323530721193,
        348569681703694236,
        4504480900646250308,
        1879493447792722501,
        1903776835985150351,
        2254331776326224169,
        2355914637511288765,
        5877050984324040355,
        7156853982837558722,
        630034316683788925,
        7011877323214027439,
        2569548525569936285,
        6221179606511727763,
        7518342859104181623,
        9012940330041424382,
        3078171300400514774,
        2133069485403307044,
        9045895164036856076,
        1704013727869055153,
        4043523965108269815,
        5772882310204761858,
        7140056047080257579,
        5198313063341071315,
        2420201020163730672,
        8753021870931183854,
        1425997325549831621,
        5075698222761332535,
        6206512662916243177,
        3873817884465805422,
        1600771696540592563,
        5261749475596256268,
        1976706489840833366,
        5986670030111034395,
        6081751036527674025,
        8787944443435723476,
        4952110543395179018,
        4765027064527162831,
        1345614005503929821,
        4081097403253782811,
        5324686745402426097,
        8568544095967929588,
        5219741208502500369,
        1324794771390488501,
        8664920732975926597,
        2428920873936490619,
        6360102853358764363,
        6257643479738398145,
        4386129712328095635,
        1092839892015207049,
        2351634539339040521,
        8610588468693408646,
        2746909918878211372,
        7157512246060364026,
        8116762340462336819,
        1470858598720885447,
        3999407537209998663,
        2947627194824871398,
        3393935753666277484,
        1133733227123638580,
        7051562004509117534,
        7518908189587431270,
        514932263576840156,
        7729771460407806878,
        2264167361527462028,
        5042769481550958411,
        2849491053637747269,
        1102981455783080301,
        1364995704159304411,
        2459950616914294856,
        8192549361792617983,
        526363624363262623,
        7069110317164898488,
        6189354718922884451,
        8930816660083986931,
        7133852426927402283,
        7509046682162276488,
        935509303526309816,
        9165692053164675008,
        8184787655928335547,
        5380120166171390053,
        9086763190023610091,
        9050773614598420749,
        8638062700480317396,
        8539592470415380105,
        8706935888943159413,
        6818310361891714637,
        6104211620022142557,
        6040552477776948136,
        8919415870744092273,
        1633199500939764423,
        2589594095669434429,
        5550130524273181452,
        3640531481322838936,
        4381888828546821451,
        7144510551460037641,
        907408557389194144,
        5176473485166497703,
        8557217129642715474,
        3967348089495227359,
        750048488752692876,
        6624680528306991817,
        5925520067107359226,
        6736302441044955823,
        8361983392152696782,
        8190910028836465944,
        2149390477307426283,
        2514528230635606587,
        2270879429344800396,
        8329392934403155950,
        5127493021180031758,
        3523492613444821751,
        7163030140714513938,
        2617703021961813102,
        3847410162181321086,
        8520480655026985678,
        8980182463265656474,
        8823516655005721326,
        4387562002793027457,
        152935407784168481,
        3198598304836432799,
        7691428753432087625,
        6350725020160109460,
        6593542946067460243,
        5021066689410609418,
        1061196409581066409,
        5987563331110678454,
        2396964830336771842,
        3983852700341195304,
        5906989894462022085,
        991037832057032876,
        3861124911635396066,
        3007446963656532169,
        5684723104405235372,
        8234449924431300690,
        8427059701692927640,
        2855372311642881197,
        5140646598587891487,
        3339965870878029682,
        6093976466378312265,
        2593831554208104803,
        3485020787142181373,
        1710939371515209343,
        4182776948419535949,
        8221365225745292637,
        8588745556227136382,
        2059328233297976992,
        950351842492269892,
        1294243534872985171,
        8293258313473872106,
        5241625948798494741,
        3285108151200296974,
        2381536148402199284,
        2693537102867751893,
        8934200656897595904,
        4825432062979130205,
        6532746640509874851,
        3367686047330981536,
        3968566209909792171,
        3878908239541211687,
        3061164835594007751,
        1756439262747169838,
        9023300517667047702,
        1468605102994116486,
        5437651421656550584,
        7901253074505957509,
        2376273090107646041,
        218427177714548537,
        6259780862397019135,
        9104022233098579196,
        278034141943567989,
        4417878622444641144,
        5056058695024207040,
        3674836322662066161,
        47103791473909118,
        999817130434475596,
        7446044937541747067,
        5268990435259850168,
        4082671229535120089,
        6055836199823770462,
        5288030007149632251,
        3385280682160083200,
        3746959213718972324,
        161420239211839628,
        228509596676881211,
        3633112025477281950,
        1718288366450622708,
        8981213613756420104,
        6361509272032941498,
        2520272508258532211,
        3768012450566212626,
        8616850716071373584,
        8395818160368956636,
        3015711825667908179,
        1718202797119379984,
        1895421867097986946,
        796791939242177065,
        8113018323636412673,
        1197936199297594771,
        2049565760656166286,
        5889631989843294732,
        1383881868672116974,
        5871090998555271175,
        7200388638287744877,
        8499541015579531874,
        6547472851702554919,
        2411876476513801318,
        394046959315000030,
        3859475948665679925,
        366119240432499195,
        571335513103826233,
        7050722647866040521,
        4717543120454663361,
        5542518166308959738,
        811585654533338408,
        6503383791693510815,
        3133720564570640748,
        6305988331328427122,
        7695703805827777133,
        8509101466335826955,
        1005559468334620853,
        4066093549314038666,
        8432088304250890031,
        8139127437965660094,
        4547915866049850157,
        2388975597955418873,
        4513194018065461924,
        897675847443613689,
        6301932299225189453,
        7163339752881179768,
        7564732925841479726,
        6933227339545949737,
        287658687282525926,
        4989036374444964466,
        6000971612443238510,
        6959484596236545329,
        8166102852907550945,
        3758823673469277900,
        1976436194418814740,
        4486976236299455413,
        3389724740299261115,
        1451781711890698170,
        343786471327386080,
        6192374030176810614,
        153585304562445894,
        2560169275811861164,
        8019372157121451358,
        6170578209644138117,
        6784415559309770968,
        3862873667715593123,
        7230372620904517937,
        935604450294373594,
        4945089392602645553,
        4026745129121058756,
        2749730982237765439,
        7094038088546402392,
        2424585159089416788,
        175886114698381010,
        8972281387458757833,
        5422992479264308135,
        8096274840550572438,
        1207030024240263135,
        7418520180028156214,
        8327507981368506624,
        8130345995129652418,
        7739393156912727390,
        8260688662007104155,
        3714505571467763692,
        6893709690131749846,
        2408666340358602644,
        8730534749958426676,
        3098806965628091423,
        8588220029231391916,
        7582929286056780303,
        4767604498787287467,
        55959053088474982,
        2099694158460295769,
        4493908662387518859,
        5571482091365138871,
        5597450299596260883,
        5384703580071347097,
        528667885429119642,
        6304590076282780771,
        4364425374762488691,
        1727328021600942959,
        2065081132138382246,
        6394177303080874702,
        6469960158071523001,
        5265063497805791987,
        177237534848636604,
        7815784936595635900,
        7678379775341887160,
        3897996216688345909,
        1577427435088690873,
        5849628140388429575,
        263102572330979691,
        1285104999937120861,
        8916987431790578092,
        6228064298896211071,
        7475128097058549792,
        4017373931546880298,
        850210544599285395,
        4370078289942915938,
        810766647496711079,
        639162666733737469,
        2824810694372134699,
        1216281388220484276,
        177334304916661415,
        2349125149064581287,
        4121151530479890798,
        1839489062056208524,
        4678164283192139450,
        3301570915526058563,
        804621090135738875,
        651058102878905525,
        4334850394183806460,
        7622299853667291963,
        4242718976879199086,
        4899758872716476522
    ];
}