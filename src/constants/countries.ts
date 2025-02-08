import type { Properties, VectorPoint } from '..';

/** A Country with its names, alpha2 and alpha3 codes */
export interface Country extends Properties {
  /** OSM ID */
  osmID?: number;
  /** Boundary ID */
  boundaryID?: number;
  /** Admin center */
  admin_centre?: number;
  /** Default name */
  name: string;
  /** ISO3166-1:alpha2 */
  alpha2: string;
  /** ISO3166-1:alpha3 */
  alpha3: string;
  /** ISO3166-1:numeric */
  numeric?: number;
  /** End link to Wikipedia location */
  wikipedia?: string;
  /** End link to English Wikipedia location */
  wikipedia_en?: string;
  /** End link to Wikidata location */
  wikidata?: string;
  /** Arabic name */
  name_ar?: string;
  /** English name */
  name_en?: string;
  /** Spanish name */
  name_es?: string;
  /** French name */
  name_fr?: string;
  /** German name */
  name_de?: string;
  /** Italian name */
  name_it?: string;
  /** Portuguese name */
  name_pt?: string;
  /** Russian name */
  name_ru?: string;
  /** Simplified Chinese name */
  name_zh_Hans?: string;
  /** Traditional Chinese name */
  name_zh_Hant?: string;
  /** Japanese name */
  name_ja?: string;
  /** Korean name */
  name_ko?: string;
  /** Vietnamese name */
  name_vi?: string;
  /** Hindi name */
  name_hi?: string;
  /** Bengali name */
  name_bn?: string;
  /** Urdu name (Pakistani and Indian) */
  name_ur?: string;
  /** Persian name (Iranian, Afghanistan, Tajikistan) */
  name_fa?: string;
  /** Turkish name (and parts of Cyprus) */
  name_tr?: string;
  /** Swahili name (Kenya, Tanzania, Uganda) */
  name_sw?: string;
  // Regional & Culturally Significant Languages
  /** Dutch name (Netherlands, Belgium, Suriname) */
  name_nl?: string;
  /** Slavic name (Largest Slavic language in the EU) */
  name_pl?: string;
  /** Greek name (Greece and Cyprus) */
  name_el?: string;
  /** Hebrew name (Israel) */
  name_he?: string;
  /** Thai name (Thailand) */
  name_th?: string;
  /** Malay name (Malaysia, Indonesia, Brunei) */
  name_ms?: string;
}

/**
 * # Countries
 *
 * ## Description
 * Lists all 195 Countries including 2 observer states (Vatican City and Palestine)
 *
 * ## Links
 * https://www.worldometers.info/geography/alphabetical-list-of-countries/
 */
export const COUNTRIES: VectorPoint<Country>[] = [
  {
    x: 66.2385139,
    y: 33.7680065,
    m: {
      name: 'Afghanistan',
      alpha2: 'AF',
      alpha3: 'AFG',
      numeric: 4,
    },
  },
] as const;

// TODO:

// 1	Afghanistan	42,647,492	652,860	65
// 2	Albania	2,791,765	27,400	102
// 3	Algeria	46,814,308	2,381,740	20
// 4	Andorra	81,938	470	174
// 5	Angola	37,885,849	1,246,700	30
// 6	Antigua and Barbuda	93,772	440	213
// 7	Argentina	45,696,159	2,736,690	17
// 8	Armenia	2,973,840	28,470	104
// 9	Australia	26,713,205	7,682,300	3
// 10	Austria	9,120,813	82,409	111
// 11	Azerbaijan	10,336,577	82,658	125
// 12	Bahamas	401,283	10,010	40
// 13	Bahrain	1,607,049	760	2,115
// 14	Bangladesh	173,562,364	130,170	1,333
// 15	Barbados	282,467	430	657
// 16	Belarus	9,056,696	202,910	45
// 17	Belgium	11,738,763	30,280	388
// 18	Belize	417,072	22,810	18
// 19	Benin	14,462,724	112,760	128
// 20	Bhutan	791,524	38,117	21
// 21	Bolivia	12,413,315	1,083,300	11
// 22	Bosnia and Herzegovina	3,164,253	51,000	62
// 23	Botswana	2,521,139	566,730	4
// 24	Brazil	211,998,573	8,358,140	25
// 25	Brunei	462,721	5,270	88
// 26	Bulgaria	6,757,689	108,560	62
// 27	Burkina Faso	23,548,781	273,600	86
// 28	Burundi	14,047,786	25,680	547
// 29	Côte d'Ivoire	31,934,230	318,000	100
// 30	Cabo Verde	524,877	4,030	130
// 31	Cambodia	17,638,801	176,520	100
// 32	Cameroon	29,123,744	472,710	62
// 33	Canada	39,742,430	9,093,510	4
// 34	Central African Republic	5,330,690	622,980	9
// 35	Chad	20,299,123	1,259,200	16
// 36	Chile	19,764,771	743,532	27
// 37	China	1,419,321,278	9,388,211	151
// 38	Colombia	52,886,363	1,109,500	48
// 39	Comoros	866,628	1,861	466
// 40	Congo (Congo-Brazzaville)	6,332,961	341,500	19
// 41	Costa Rica	5,129,910	51,060	100
// 42	Croatia	3,875,325	55,960	69
// 43	Cuba	10,979,783	106,440	103
// 44	Cyprus	1,358,282	9,240	147
// 45	Czechia (Czech Republic)	10,735,859	77,240	139
// 46	Democratic Republic of the Congo	109,276,265	2,267,050	48
// 47	Denmark	5,977,412	42,430	141
// 48	Djibouti	1,168,722	23,180	50
// 49	Dominica	66,205	750	88
// 50	Dominican Republic	11,427,557	48,320	236
// 51	Ecuador	18,135,478	248,360	73
// 52	Egypt	116,538,258	995,450	117
// 53	El Salvador	6,338,193	20,720	306
// 54	Equatorial Guinea	1,892,516	28,050	67
// 55	Eritrea	3,535,603	101,000	35
// 56	Estonia	1,360,546	42,390	32
// 57	Eswatini (fmr. "Swaziland")	1,242,822	17,200	72
// 58	Ethiopia	132,059,767	1,000,000	132
// 59	Fiji	928,784	18,270	51
// 60	Finland	5,617,310	303,890	18
// 61	France	66,548,530	547,557	122
// 62	Gabon	2,538,952	257,670	10
// 63	Gambia	2,759,988	10,120	273
// 64	Georgia	3,807,670	69,490	55
// 65	Germany	84,552,242	348,560	243
// 66	Ghana	34,427,414	227,540	151
// 67	Greece	10,047,817	128,900	78
// 68	Grenada	117,207	340	345
// 69	Guatemala	18,406,359	107,160	172
// 70	Guinea	14,754,785	245,720	60
// 71	Guinea-Bissau	2,201,352	28,120	78
// 72	Guyana	831,087	196,850	4
// 73	Haiti	11,772,557	27,560	427
// 74	Holy See	496	0	1,240
// 75	Honduras	10,825,703	111,890	97
// 76	Hungary	9,676,135	90,530	107
// 77	Iceland	393,396	100,250	4
// 78	India	1,450,935,791	2,973,190	488
// 79	Indonesia	283,487,931	1,811,570	156
// 80	Iran	91,567,738	1,628,550	56
// 81	Iraq	46,042,015	434,320	106
// 82	Ireland	5,255,017	68,890	76
// 83	Israel	9,387,021	21,640	434
// 84	Italy	59,342,867	294,140	202
// 85	Jamaica	2,839,175	10,830	262
// 86	Japan	123,753,041	364,555	339
// 87	Jordan	11,552,876	88,780	130
// 88	Kazakhstan	20,592,571	2,699,700	8
// 89	Kenya	56,432,944	569,140	99
// 90	Kiribati	134,518	810	166
// 91	Kuwait	4,934,507	17,820	277
// 92	Kyrgyzstan	7,186,009	191,800	37
// 93	Laos	7,769,819	230,800	34
// 94	Latvia	1,871,871	62,200	30
// 95	Lebanon	5,805,962	10,230	568
// 96	Lesotho	2,337,423	30,360	77
// 97	Liberia	5,612,817	96,320	58
// 98	Libya	7,381,023	1,759,540	4
// 99	Liechtenstein	39,870	160	249
// 100	Lithuania	2,859,110	62,674	46
// 101	Luxembourg	673,036	2,590	260
// 102	Madagascar	31,964,956	581,795	55
// 103	Malawi	21,655,286	94,280	230
// 104	Malaysia	35,557,673	328,550	108
// 105	Maldives	527,799	300	1,759
// 106	Mali	24,478,595	1,220,190	20
// 107	Malta	539,607	320	1,686
// 108	Marshall Islands	37,548	180	209
// 109	Mauritania	5,169,395	1,030,700	5
// 110	Mauritius	1,271,169	2,030	626
// 111	Mexico	130,861,007	1,943,950	67
// 112	Micronesia	526,923	700	753
// 113	Moldova	3,034,961	32,850	92
// 114	Monaco	38,631	1	25,927
// 115	Mongolia	3,475,540	1,553,560	2
// 116	Montenegro	638,479	13,450	47
// 117	Morocco	38,081,173	446,300	85
// 118	Mozambique	34,631,766	786,380	44
// 119	Myanmar (formerly Burma)	54,500,091	653,290	83
// 120	Namibia	3,030,131	823,290	4
// 121	Nauru	11,947	20	597
// 122	Nepal	29,651,054	143,350	207
// 123	Netherlands	18,228,742	33,720	541
// 124	New Zealand	5,213,944	263,310	20
// 125	Nicaragua	6,916,140	120,340	57
// 126	Niger	27,032,412	1,266,700	21
// 127	Nigeria	232,679,478	910,770	255
// 128	North Korea	26,498,823	120,410	220
// 129	North Macedonia	1,823,009	25,220	72
// 130	Norway	5,576,660	365,268	15
// 131	Oman	5,281,538	309,500	17
// 132	Pakistan	251,269,164	770,880	326
// 133	Palau	17,695	460	38
// 134	Palestine State	5,495,443	6,020	913
// 135	Panama	4,515,577	74,340	61
// 136	Papua New Guinea	10,576,502	452,860	23
// 137	Paraguay	6,929,153	397,300	17
// 138	Peru	34,217,848	1,280,000	27
// 139	Philippines	115,843,670	298,170	389
// 140	Poland	38,539,201	306,230	126
// 141	Portugal	10,425,292	91,590	114
// 142	Qatar	3,048,423	11,610	263
// 143	Romania	19,015,088	230,170	83
// 144	Russia	144,820,423	16,376,870	9
// 145	Rwanda	14,256,567	24,670	578
// 146	Saint Kitts and Nevis	46,843	260	180
// 147	Saint Lucia	179,744	610	295
// 148	Saint Vincent and the Grenadines	100,616	390	258
// 149	Samoa	218,019	2,830	77
// 150	San Marino	33,581	60	560
// 151	Sao Tome and Principe	235,536	960	245
// 152	Saudi Arabia	33,962,757	2,149,690	16
// 153	Senegal	18,501,984	192,530	96
// 154	Serbia	6,736,216	87,460	77
// 155	Seychelles	130,418	460	284
// 156	Sierra Leone	8,642,022	72,180	120
// 157	Singapore	5,832,387	700	8,332
// 158	Slovakia	5,506,760	48,088	115
// 159	Slovenia	2,118,697	20,140	105
// 160	Solomon Islands	819,198	27,990	29
// 161	Somalia	19,009,151	627,340	30
// 162	South Africa	64,007,187	1,213,090	53
// 163	South Korea	51,717,590	97,230	532
// 164	South Sudan	11,943,408	610,952	20
// 165	Spain	47,910,526	498,800	96
// 166	Sri Lanka	23,103,565	62,710	368
// 167	Sudan	50,448,963	1,765,048	29
// 168	Suriname	634,431	156,000	4
// 169	Sweden	10,606,999	410,340	26
// 170	Switzerland	8,921,981	39,516	226
// 171	Syria	24,672,760	183,630	134
// 172	Tajikistan	10,590,927	139,960	76
// 173	Tanzania	68,560,157	885,800	77
// 174	Thailand	71,668,011	510,890	140
// 175	Timor-Leste	1,400,638	14,870	94
// 176	Togo	9,515,236	54,390	175
// 177	Tonga	104,175	720	145
// 178	Trinidad and Tobago	1,507,782	5,130	294
// 179	Tunisia	12,277,109	155,360	79
// 180	Turkey	87,473,805	769,630	114
// 181	Turkmenistan	7,494,498	469,930	16
// 182	Tuvalu	9,646	30	322
// 183	Uganda	50,015,092	199,810	250
// 184	Ukraine	37,860,221	579,320	65
// 185	United Arab Emirates	11,027,129	83,600	132
// 186	United Kingdom	69,138,192	241,930	286
// 187	United States of America	345,426,571	9,147,420	38
// 188	Uruguay	3,386,588	175,020	19
// 189	Uzbekistan	36,361,859	425,400	85
// 190	Vanuatu	327,777	12,190	27
// 191	Venezuela	28,405,543	882,050	32
// 192	Vietnam	100,987,686	310,070	326
// 193	Yemen	40,583,164	527,970	77
// 194	Zambia	21,314,956	743,390	29
// 195	Zimbabwe	16,634,373	386,850	43
