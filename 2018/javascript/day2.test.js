import test from 'ava';

function repeats(id) {
  let m = id.split('')
    .reduce(
      (acc, current) => {
        acc[current] = acc[current] || 0;
        acc[current]++;
        return acc;
      }, {});
  let counts = Object.values(m);
  if (counts.includes(2) && counts.includes(3)) {
    return [1, 1];
  } else if (counts.includes(2)) {
    return [1, 0];
  } else if (counts.includes(3)) {
    return [0, 1];
  } else {
    return [0, 0];
  }
}

function repeatSum(repeats) {
  return repeats.reduce(
    (acc, curr) => {
      return [acc[0] + curr[0], acc[1] + curr[1]];
    }, [0, 0]
  );
}

function checkSum(ids) {
  let sum = repeatSum(ids.map(repeats));
  return sum[0] * sum[1];
}

test('abcdef returns 0 0', t => {
  t.deepEqual(repeats('abcdef'), [0, 0]);
});

test('bababc returns 1 1', t => {
  t.deepEqual(repeats('bababc'), [1, 1]);
});

test('abbcde returns 1 0', t => {
  t.deepEqual(repeats('abbcde'), [1, 0]);
});

test('abcccd returns 0 1', t => {
  t.deepEqual(repeats('abcccd'), [0, 1]);
});

test('repeatSum given sequence returns sum of each', t => {
  let result = repeatSum([[1, 0], [1, 1], [0, 0], [0, 1], [1, 0]]);
  t.deepEqual(result, [3, 2]);
});

test('checkSum given multiple ids returns checksum', t => {
  let ids = [
    "abcdef",
    "bababc",
    "abbcde",
    "abcccd",
    "aabcdd",
    "abcdee",
    "ababab"
  ]
  let result = checkSum(ids);
  t.is(result, 12);
});

test('part1 puzzle', t => {
  let ids = ["qcsnyvpigkxmrdawlfdefotxbh", "qcsnyvligkymrdawljujfotxbh", "qmsnyvpigkzmrnawzjuefotxbh", "qosnyvpigkzmrnawljuefouxbh", "qcsnhlpigkzmrtawljuefotxbh", "qcsnyvpigkzmrdapljuyfotxih", "qcsnbvpiokzmrdawljuerotxbh", "qcfnyvmigkzmrdawljuefotdbh", "qcsnynpigkzmrdawljuefptxbp", "qcsgyapigkzmrdawljuafotxbh", "qcsnyvpigkzmrdapljueeotibh", "qcfnyvpigkzmndawljuwfotxbh", "qzsayvpigkzmrdawijuefotxbh", "qcsnsvpiekzmrdawljfefotxbh", "ncsnyvpigkzmrdaaljuefotxzh", "qssnyvpigkzmrdawljuefotobg", "qcshyipigkzmrdajljuefotxbh", "qcsnyvtigkzmrdawljgeaotxbh", "qcsnkvpxgkzmrdawljuefltxbh", "qcsnyvpiikzmrdawljuwfoqxbh", "qcsnybpigwzmqdawljuefotxbh", "qcsiyvpipkzbrdawljuefotxbh", "qldnyvpigkzmrdzwljuefotxbh", "qcsnyvpwgkzcrdawljuefmtxbh", "qcsnyvnigkzmrdahmjuefotxbh", "qcsnydpigkzmrdazljuefotxnh", "qcsqyvavgkzmrdawljuefotxbh", "ucsnyvpigkzmrdawljuefocxwh", "qcsnivpigrzmrdawljuefouxbh", "tcsnyvpibkzmrdawlkuefotxbh", "qcstytpigkzmrdawsjuefotxbh", "qcynyvpigkzmrdawlluefotjbh", "qcstyvpigkqrrdawljuefotxbh", "icsnyvpizkzmrcawljuefotxbh", "qcsnyvpimkzmrdavljuezotxbh", "qvsnoupigkzmrdawljuefotxbh", "qcsnyvpigkzmrdawwjuefftxgh", "qcpnyvpijkzmrdvwljuefotxbh", "qcsnyvpigkzmxdakdjuefotxbh", "jcsvyvpigkqmrdawljuefotxbh", "qcwnyvpigczmrsawljuefotxbh", "qcsnyvpdgkzmrdawljuefoixbm", "qysnyvpigkzmrdmwljuefotxbp", "qcsnavpigkzmrdaxajuefotxbh", "qcsfkvpigkzmrdawlcuefotxbh", "qcsnyvpigkvmrdawljcefotpbh", "qcsnyvpiqkkmrdawlvuefotxbh", "qhsnyvpigkzmrdawnjuedotxbh", "qasnlvpigkzmrdawljuefotxkh", "qgsnyvpigkzmrdabpjuefotxbh", "jcsnyvdigkzmrmawljuefotxbh", "qcsnivpigkzmrdawljuefonxth", "qcsnyjpigkzmrdawljgefotxmh", "qcstyvpigkzmrdacljuefovxbh", "qcsnvvpigkzmrdawljuewotrbh", "qcsnyvaigdzmrdawljueuotxbh", "qcsnyvpegkzmwdawljzefotxbh", "qcsnevpngkzmrdawlouefotxbh", "qcsnuvpigozmrdawljuefotdbh", "qgsnyvpigkzmqdayljuefotxbh", "qcsnyvpigkzmrdcwdjuofotxbh", "qcnnyvpigkzmrzawljuefstxbh", "qlsgyvpigkzmrdtwljuefotxbh", "qcsnyfpigkzlroawljuefotxbh", "qcsnkvwigkzmrdowljuefotxbh", "qcsnrvpigkzmrdawljuvfltxbh", "qcsnyvpigkzvreawljuefotxmh", "qcsrgvpigkzmrdawliuefotxbh", "qysnyvpigkzmrdawlxaefotxbh", "qcsnyvpigizmrdlwljuefotxbi", "qzsnyvpitkzmrdawljuefbtxbh", "qzgnyvpigkzmrdawljuefotxih", "qcsnyvpigkzmrdawlguefvtxbb", "qcsnyvpigkzmidawljuefouxjh", "qksnyvpigkzmrdawlruefotxhh", "qcsnyvpinkzmrdaaljuefotxah", "qcsnxvpigkzjrdawljuefhtxbh", "qcsnyvpigkzardawlgueuotxbh", "qcsnyvpiakzmrdpwljuefotxbt", "qcsnyvpigkzmrdawkjuefotxgb", "qcsnyvpigkzmrdawljuehocsbh", "qcsnsvpigktmrdawljuefotxvh", "qusnrvpigkzrrdawljuefotxbh", "qcsnyhiigkzmrdawrjuefotxbh", "qcsnavpigkzmrdawlfuefotxbz", "qcsnyvpigkzmmdamsjuefotxbh", "qcsnyvzigkzmrdcwljmefotxbh", "qcsnyvpigkzmriawljuefotbbe", "qcsnyvpigksmrdawljaefotxbd", "qcsnyvpigkzfrdawljuefoxxmh", "qcsnyvpygkrmrdawljuefotxbi", "qcsngvwigfzmrdawljuefotxbh", "qcsnyvpigkmkrdauljuefotxbh", "qcsnyvpigxzmrdgwljuefwtxbh", "qconyapigkzmrdaxljuefotxbh", "qcsnydpigkzwrdawljulfotxbh", "qcsnyvpimkzmmdawljuefotxch", "qcsnkspigkzmrdawgjuefotxbh", "qcsnyvpigkzmrdhwljfefbtxbh", "qcsnyipijkztrdawljuefotxbh", "qcseyvpigkrhrdawljuefotxbh", "qcsnyvpivkzmrdawljuefottbb", "qcsnyvpigkzmrdawlouefcjxbh", "qcsnyvpigkzmrgayljuefotxbm", "qcsnyvpvgkzmrdawrjujfotxbh", "qcsnyvpigkzmndawljuefqtxch", "qcsnyvpigbzmrdawljuefotibg", "qcsnyvpigkzmseawljuefotxbv", "qcsnwvpigkzmraawnjuefotxbh", "mcsnyvpiqkzmrdawljuefotlbh", "bcsnyvpigczmrdmwljuefotxbh", "qcsnyvpigkzmrtawljuegntxbh", "qcsnyvpijkzmrdawlmrefotxbh", "qdsnyvpfgkzmrdawljuekotxbh", "qcsnyvpigkzmrdawcjfegotxbh", "qcslyvphgkrmddawljuefotxbh", "qcsnyvpigkzmsdawkjuefojxbh", "qzsnyvpigkzmrzawljuefmtxbh", "qcsnyvpqgkzmcdawljuefttxbh", "qcsnyvpbgkpmrdawljuefoqxbh", "qcsnyvpigkemrdywljmefotxbh", "qcsnyypigkzmrdawljmefotxwh", "jcsnyvhwgkzmrdawljuefotxbh", "qcsnyvpigkzmrdawljurlotxwh", "qcsnnvpigzzmrdawljuefotwbh", "hcsnyvpigkzmrdarljuefitxbh", "qcsnyvpilkzmrfawljuefotsbh", "qcsnynpigkzmldawijuefotxbh", "qcsnyvpkgkjmrdawljuefotxlh", "qcsnylpigkzprdawljgefotxbh", "qcsnyvpigkzmrrawljnefohxbh", "qcsnivpigkzmrqawlbuefotxbh", "qcsgyvpigkzmrfawljuefotbbh", "qccuyvpigkzmrdawyjuefotxbh", "gcsnyvpigkzjrdawljuefotxby", "qcsmyvpiekzbrdawljuefotxbh", "qcsnyvpzgkrmrdawljuefotxbs", "qesnyvpigkzmpdqwljuefotxbh", "qcsnyvpigqzmrdawljuefutibh", "qcdnyvpigkzirdawljfefotxbh", "qcsnyvpiukzmrcrwljuefotxbh", "qcsnbvpickzmrdswljuefotxbh", "qcsnyvpighzmrpadljuefotxbh", "qccnyvpigkzmrdawljudxotxbh", "qcsnyvpigkzmrdabljuesotxlh", "qcsnyvpigkzmrrawlruefozxbh", "qconyzpigkzmrdawljuefotjbh", "qclnyvpigkzmrdxwljuefotbbh", "qcsnygpigkzmrdawlhuefooxbh", "qcsnyvpigkzmvdawljuefntxnh", "qcskyvpigkzmreawljuefotubh", "qrsnyvpxgkzmrdawljuefotxbz", "qclnyvpigtamrdawljuefotxbh", "qcsnyvpigkzmrdawojxefoyxbh", "qcsnyvpinkzmrdakljuwfotxbh", "qcsnyvpiykzmedawljuefgtxbh", "qcsayvpigkcmrdawijuefotxbh", "qcsnyvuiekzmrdamljuefotxbh", "qcdnyvpigkzmrdawnjuefoxxbh", "qcsnfvpwgszmrdawljuefotxbh", "qcsnycpigkzmrdawljqefotxih", "qcslyvphgkrmrdawljuefotxbh", "ecsnyvpigkzmrdawykuefotxbh", "qcsayvpigkzmraawljuekotxbh", "qcsnyvpigkdmrdawljuewofxbh", "qcznyvpigkzqrdawljuefotxnh", "qcsnyvplgkzmrdawljiefotlbh", "qcsnyvpigkzmroewljuefotbbh", "qcvnyvpigkzvrdawujuefotxbh", "qcanyypigkzmrdaeljuefotxbh", "qcsnyvwigkzmrdewljuefotxqh", "qcsryvpigkvmrdawljuefotabh", "pcsnyvpigkwmrdawljueforxbh", "qcsncvpigkzmrdawljuefotwmh", "qcsnyvpigozmrdawljudfozxbh", "qcsnynpigkzmrbawhjuefotxbh", "qcsnyvuigkzmrqawljuefotxch", "qcsnyvpickzmrdawljueeofxbh", "qcsnyvpigkzgrdawljueiouxbh", "qcsnyvpigkztrdawljuxnotxbh", "qcsnyvpigwzvrdawljfefotxbh", "qcsnyvpilkzmrdawljuefotxcz", "qcsnjvpigkzmrdawljuefoywbh", "qhsnyvpigyzmrdawljuhfotxbh", "qcsnyvpirkzmfdawljuffotxbh", "qcsjyvpigkzmvdawljuefotxzh", "qcszivpirkzmrdawljuefotxbh", "qwsnyvpigkzmtdawljuefetxbh", "qcrntvpigkzordawljuefotxbh", "qrsnyvpigkzmsdawljrefotxbh", "qcsnyviivkzmrdazljuefotxbh", "ecsnyvpigkzmrdawyjuefotxbw", "qnsnyvpkgkzmrdawljueqotxbh", "qcsyyppigkzmrdawljuefotxba", "qcsnyvpigkzhrdpwljuefouxbh", "ucsnyvpigkzmrdawojuefouxbh", "qysnyvpigkzmrdawljukfotxbd", "qcjnyvpigkzmrdalljfefotxbh", "fcsnyapigkmmrdawljuefotxbh", "qcnnkvpigkzmrdawljuefctxbh", "ocsnyvpigkzmsdawljuefotxbl", "qcsnyvpiakomrdawpjuefotxbh", "qcsnyvpigkzmrdawljvefbtxwh", "qcsnuvpigkzmvdfwljuefotxbh", "qcsnyapihkzmrdagljuefotxbh", "qzsnyvpigkzmrdawtjuefotxgh", "qcsnyvpigkzmrdawljuefomyah", "ocsnyvpigkzqrdawljuefotxbt", "qnsnyvpigkzmrdawljvevotxbh", "icsnyvpigkzmrdawljuefntxbt", "qcsnyvpigkzdrdawljuefotbbm", "scsnyvpigkzmrgawljuofotxbh", "qcsnydpigkzmrdowljuefotkbh", "qcsnyvtikkzmrdawljuefolxbh", "qcsiyvpigkcmrddwljuefotxbh", "qyrnyvpigkzmodawljuefotxbh", "pcsndvpfgkzmrdawljuefotxbh", "qcsnyvkigkhmriawljuefotxbh", "qcsnyvpigkzmsdmwlkuefotxbh", "dosnyvpigkzmrdawdjuefotxbh", "qcnnnvpigkzmrdzwljuefotxbh", "qcsnyvpivkumrdailjuefotxbh", "qcsnyvpigkzmrdswljuzfotxbz", "qcscynpigkzmrdawljuefotxbc", "qeanyvpigkzmrdawijuefotxbh", "qclnylpigkzmrdawljuefotxyh", "qcsnyvpigkzmrdawljbefowxbp", "qcsnyvpagkzmrdawljuefolebh", "qxsiyvpigkzmrdawljuefotxgh", "qcsnyvpigkynrdawljuefoqxbh", "qcsnevpigkzmrdxwgjuefotxbh", "qcsnyvpdgkzlrdawljeefotxbh", "qcsnyvpigkzmrgawljxbfotxbh", "ecsnyvpigkzmrdbwbjuefotxbh", "qcsnyvpigkzmraawujuefocxbh", "qcsnyvpihkzmrdawljuefouxbn", "fgsqyvpigkzmrdawljuefotxbh", "qcsnyvpigkmmrdawajuefotnbh", "qcsnyvvigkzmrdahljudfotxbh", "qcsnyvpixkzmrdqwljutfotxbh", "ncsnyvpickzmrdawljuehotxbh", "qcsnyvpizkzmrdawlpuefotxbp", "wcsnyvfigkzmrdakljuefotxbh", "qcsnyvpigkznrdhwljupfotxbh", "jcsnyvpigkpmzdawljuefotxbh", "qcsnyppigkkmrdawljujfotxbh", "qcsnyvpigkumrdaeljuefodxbh", "qcsnyvhigkzmrdrwljuefodxbh", "qcsnyvpigkacrdawtjuefotxbh", "qcsnyvpigkzmylawlquefotxbh"];
  let result = checkSum(ids);
  t.is(result, 6422);
});