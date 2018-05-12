require "spec_helper"

RSpec.describe AdventOfCode::Day2 do
  it 'is a class' do
    expect(described_class).to be_a Class
  end

  spreadsheet = [[104,240,147,246,123,175,372,71,116,230,260,118,202,270,277,292], 
    [740,755,135,205,429,822,844,90,828,115,440,805,526,91,519,373], 
    [1630,991,1471,1294,52,1566,50,1508,1367,1489,55,547,342,512,323,51], 
    [1356,178,1705,119,1609,1409,245,292,1434,694,405,1692,247,193,1482,1407], 
    [2235,3321,3647,212,1402,3711,3641,1287,2725,692,1235,3100,123,144,104,101], 
    [1306,1224,1238,186,751,734,1204,1275,366,149,1114,166,1118,239,153,943], 
    [132,1547,1564,512,2643,2376,2324,2159,1658,107,1604,145,2407,131,2073,1878], 
    [1845,91,1662,108,92,1706,1815,1797,1728,1150,1576,83,97,547,1267,261], 
    [78,558,419,435,565,107,638,173,93,580,338,52,633,256,377,73], 
    [1143,3516,4205,3523,148,401,3996,3588,300,1117,2915,1649,135,134,182,267], 
    [156,2760,1816,2442,2985,990,2598,1273,167,821,138,141,2761,2399,1330,1276], 
    [3746,3979,2989,161,4554,156,3359,173,3319,192,3707,264,762,2672,4423,2924], 
    [3098,4309,4971,5439,131,171,5544,595,154,571,4399,4294,160,6201,4329,5244], 
    [728,249,1728,305,2407,239,691,2241,2545,1543,55,2303,1020,753,193,1638], 
    [260,352,190,877,118,77,1065,1105,1085,1032,71,87,851,56,1161,667], 
    [1763,464,182,1932,1209,640,545,931,1979,197,1774,174,2074,1800,939,161]]

  describe '.high_low' do
    it 'solves the example spreadsheet of part 1' do
      expect(AdventOfCode::Day2.high_low([[5,1,9,5], [7,5,3], [2,4,6,8]])).to eq(18)
    end
    
    it 'solves the part 1 challenge' do
        expect(AdventOfCode::Day2.high_low(spreadsheet)).to eq(36766)
    end

  end

  describe '.even_division' do
    it 'solves the example spreadsheet of part 2' do
      expect(AdventOfCode::Day2.even_division([[5,9,2,8], [9,4,7,3], [3,8,6,5]])).to eq(9)
    end

    it 'solves the part 2 challenge' do
        expect(AdventOfCode::Day2.even_division(spreadsheet)).to eq(261)
    end
  end
end