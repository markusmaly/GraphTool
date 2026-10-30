//! Embedded open datasets shipped with ggplot2 (and base R `datasets`).
//!
//! The CSV payloads are the canonical datasets as distributed by the R
//! packages (via the Rdatasets CSV mirror), embedded verbatim so the tool
//! works fully offline. Large datasets (diamonds, economics, midwest) are
//! embedded as deterministic stratified samples; mtcars, mpg, msleep and
//! presidential are embedded in full.

/// One embedded dataset.
#[derive(Debug, Clone, Copy)]
pub struct Dataset {
    /// Short slug, e.g. `"mtcars"`.
    pub name: &'static str,
    /// R package the dataset ships with, e.g. `"ggplot2"`.
    pub package: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Canonical CSV text including the header row.
    pub csv: &'static str,
}

/// All embedded datasets.
pub fn all() -> &'static [Dataset] {
    DATASETS
}

/// Look up a dataset by slug.
pub fn get(name: &str) -> Option<&'static Dataset> {
    DATASETS.iter().find(|d| d.name == name)
}

static DATASETS: &[Dataset] = &[
    Dataset { name: "mtcars", package: "datasets", description: "Motor Trend 1973 car road tests (32 cars, 11 variables)", csv: r##"
rownames,mpg,cyl,disp,hp,drat,wt,qsec,vs,am,gear,carb
Mazda RX4,21,6,160,110,3.9,2.62,16.46,0,1,4,4
Mazda RX4 Wag,21,6,160,110,3.9,2.875,17.02,0,1,4,4
Datsun 710,22.8,4,108,93,3.85,2.32,18.61,1,1,4,1
Hornet 4 Drive,21.4,6,258,110,3.08,3.215,19.44,1,0,3,1
Hornet Sportabout,18.7,8,360,175,3.15,3.44,17.02,0,0,3,2
Valiant,18.1,6,225,105,2.76,3.46,20.22,1,0,3,1
Duster 360,14.3,8,360,245,3.21,3.57,15.84,0,0,3,4
Merc 240D,24.4,4,146.7,62,3.69,3.19,20,1,0,4,2
Merc 230,22.8,4,140.8,95,3.92,3.15,22.9,1,0,4,2
Merc 280,19.2,6,167.6,123,3.92,3.44,18.3,1,0,4,4
Merc 280C,17.8,6,167.6,123,3.92,3.44,18.9,1,0,4,4
Merc 450SE,16.4,8,275.8,180,3.07,4.07,17.4,0,0,3,3
Merc 450SL,17.3,8,275.8,180,3.07,3.73,17.6,0,0,3,3
Merc 450SLC,15.2,8,275.8,180,3.07,3.78,18,0,0,3,3
Cadillac Fleetwood,10.4,8,472,205,2.93,5.25,17.98,0,0,3,4
Lincoln Continental,10.4,8,460,215,3,5.424,17.82,0,0,3,4
Chrysler Imperial,14.7,8,440,230,3.23,5.345,17.42,0,0,3,4
Fiat 128,32.4,4,78.7,66,4.08,2.2,19.47,1,1,4,1
Honda Civic,30.4,4,75.7,52,4.93,1.615,18.52,1,1,4,2
Toyota Corolla,33.9,4,71.1,65,4.22,1.835,19.9,1,1,4,1
Toyota Corona,21.5,4,120.1,97,3.7,2.465,20.01,1,0,3,1
Dodge Challenger,15.5,8,318,150,2.76,3.52,16.87,0,0,3,2
AMC Javelin,15.2,8,304,150,3.15,3.435,17.3,0,0,3,2
Camaro Z28,13.3,8,350,245,3.73,3.84,15.41,0,0,3,4
Pontiac Firebird,19.2,8,400,175,3.08,3.845,17.05,0,0,3,2
Fiat X1-9,27.3,4,79,66,4.08,1.935,18.9,1,1,4,1
Porsche 914-2,26,4,120.3,91,4.43,2.14,16.7,0,1,5,2
Lotus Europa,30.4,4,95.1,113,3.77,1.513,16.9,1,1,5,2
Ford Pantera L,15.8,8,351,264,4.22,3.17,14.5,0,1,5,4
Ferrari Dino,19.7,6,145,175,3.62,2.77,15.5,0,1,5,6
Maserati Bora,15,8,301,335,3.54,3.57,14.6,0,1,5,8
Volvo 142E,21.4,4,121,109,4.11,2.78,18.6,1,1,4,2
"## },
    Dataset { name: "diamonds", package: "ggplot2", description: "Prices of 53,940 diamonds (29-row stratified sample embedded)", csv: r##"
rownames,carat,cut,color,clarity,depth,table,price,x,y,z
1,0.23,Ideal,E,SI2,61.5,55,326,3.95,3.98,2.43
1861,0.8,Ideal,F,SI2,61.8,59,3066,5.94,5.96,3.68
3721,0.3,Very Good,E,VS1,63,57,568,4.25,4.29,2.69
5581,1,Premium,H,SI2,61,61,3864,6.31,6.27,3.84
7441,0.94,Premium,F,SI2,62.2,59,4230,6.27,6.23,3.89
9301,1.15,Ideal,H,SI2,62.6,55,4572,6.74,6.65,4.19
11161,1.02,Good,D,SI1,56.9,62,4948,6.64,6.69,3.79
13021,0.3,Very Good,E,VS2,61.6,54,600,4.29,4.32,2.65
14881,1.09,Premium,H,VS2,62.3,57,5993,6.63,6.53,4.1
16741,1.01,Premium,E,VS2,62,59,6666,6.44,6.46,4
18601,1.12,Very Good,E,VS2,62.8,57,7589,6.61,6.64,4.16
20461,1.16,Ideal,G,VVS2,62,56,8803,6.75,6.71,4.17
22321,1.57,Ideal,E,SI2,61.4,56,10362,7.44,7.52,4.59
24181,1.52,Ideal,F,SI1,61.5,55,12364,7.41,7.44,4.57
26041,1.5,Premium,D,VS2,61.8,60,15240,7.37,7.3,4.53
27901,0.4,Good,G,SI1,63.4,58,655,4.66,4.71,2.97
29761,0.35,Premium,I,VS1,61.8,58,709,4.53,4.5,2.79
31621,0.32,Ideal,I,SI1,61.2,56,450,4.4,4.44,2.7
33481,0.32,Premium,F,VS2,60.6,59,828,4.44,4.4,2.68
35341,0.42,Premium,H,SI1,62.1,59,898,4.81,4.78,2.98
37201,0.33,Ideal,F,VVS2,60.9,56,974,4.47,4.5,2.73
39061,0.7,Fair,H,I1,66.5,57,1058,5.51,5.4,3.64
40921,0.29,Very Good,E,VS1,60.4,58,499,4.28,4.33,2.6
42781,0.41,Premium,D,VVS1,62.2,58,1349,4.71,4.77,2.95
44641,0.24,Very Good,D,VVS2,58.8,58,521,4.05,4.08,2.39
46501,0.54,Ideal,E,SI1,61.5,55,1778,5.24,5.26,3.23
48361,0.56,Very Good,E,VS2,61.1,57,1963,5.33,5.37,3.27
50221,0.7,Very Good,F,SI1,61.7,61,2230,5.67,5.7,3.51
52081,0.78,Ideal,J,VS2,62.3,55,2456,5.88,5.91,3.67
"## },
    Dataset { name: "mpg", package: "ggplot2", description: "EPA fuel economy data for 38 car models 1999-2008 (234 rows)", csv: r##"
rownames,manufacturer,model,displ,year,cyl,trans,drv,cty,hwy,fl,class
1,audi,a4,1.8,1999,4,auto(l5),f,18,29,p,compact
2,audi,a4,1.8,1999,4,manual(m5),f,21,29,p,compact
3,audi,a4,2,2008,4,manual(m6),f,20,31,p,compact
4,audi,a4,2,2008,4,auto(av),f,21,30,p,compact
5,audi,a4,2.8,1999,6,auto(l5),f,16,26,p,compact
6,audi,a4,2.8,1999,6,manual(m5),f,18,26,p,compact
7,audi,a4,3.1,2008,6,auto(av),f,18,27,p,compact
8,audi,a4 quattro,1.8,1999,4,manual(m5),4,18,26,p,compact
9,audi,a4 quattro,1.8,1999,4,auto(l5),4,16,25,p,compact
10,audi,a4 quattro,2,2008,4,manual(m6),4,20,28,p,compact
11,audi,a4 quattro,2,2008,4,auto(s6),4,19,27,p,compact
12,audi,a4 quattro,2.8,1999,6,auto(l5),4,15,25,p,compact
13,audi,a4 quattro,2.8,1999,6,manual(m5),4,17,25,p,compact
14,audi,a4 quattro,3.1,2008,6,auto(s6),4,17,25,p,compact
15,audi,a4 quattro,3.1,2008,6,manual(m6),4,15,25,p,compact
16,audi,a6 quattro,2.8,1999,6,auto(l5),4,15,24,p,midsize
17,audi,a6 quattro,3.1,2008,6,auto(s6),4,17,25,p,midsize
18,audi,a6 quattro,4.2,2008,8,auto(s6),4,16,23,p,midsize
19,chevrolet,c1500 suburban 2wd,5.3,2008,8,auto(l4),r,14,20,r,suv
20,chevrolet,c1500 suburban 2wd,5.3,2008,8,auto(l4),r,11,15,e,suv
21,chevrolet,c1500 suburban 2wd,5.3,2008,8,auto(l4),r,14,20,r,suv
22,chevrolet,c1500 suburban 2wd,5.7,1999,8,auto(l4),r,13,17,r,suv
23,chevrolet,c1500 suburban 2wd,6,2008,8,auto(l4),r,12,17,r,suv
24,chevrolet,corvette,5.7,1999,8,manual(m6),r,16,26,p,2seater
25,chevrolet,corvette,5.7,1999,8,auto(l4),r,15,23,p,2seater
26,chevrolet,corvette,6.2,2008,8,manual(m6),r,16,26,p,2seater
27,chevrolet,corvette,6.2,2008,8,auto(s6),r,15,25,p,2seater
28,chevrolet,corvette,7,2008,8,manual(m6),r,15,24,p,2seater
29,chevrolet,k1500 tahoe 4wd,5.3,2008,8,auto(l4),4,14,19,r,suv
30,chevrolet,k1500 tahoe 4wd,5.3,2008,8,auto(l4),4,11,14,e,suv
31,chevrolet,k1500 tahoe 4wd,5.7,1999,8,auto(l4),4,11,15,r,suv
32,chevrolet,k1500 tahoe 4wd,6.5,1999,8,auto(l4),4,14,17,d,suv
33,chevrolet,malibu,2.4,1999,4,auto(l4),f,19,27,r,midsize
34,chevrolet,malibu,2.4,2008,4,auto(l4),f,22,30,r,midsize
35,chevrolet,malibu,3.1,1999,6,auto(l4),f,18,26,r,midsize
36,chevrolet,malibu,3.5,2008,6,auto(l4),f,18,29,r,midsize
37,chevrolet,malibu,3.6,2008,6,auto(s6),f,17,26,r,midsize
38,dodge,caravan 2wd,2.4,1999,4,auto(l3),f,18,24,r,minivan
39,dodge,caravan 2wd,3,1999,6,auto(l4),f,17,24,r,minivan
40,dodge,caravan 2wd,3.3,1999,6,auto(l4),f,16,22,r,minivan
41,dodge,caravan 2wd,3.3,1999,6,auto(l4),f,16,22,r,minivan
42,dodge,caravan 2wd,3.3,2008,6,auto(l4),f,17,24,r,minivan
43,dodge,caravan 2wd,3.3,2008,6,auto(l4),f,17,24,r,minivan
44,dodge,caravan 2wd,3.3,2008,6,auto(l4),f,11,17,e,minivan
45,dodge,caravan 2wd,3.8,1999,6,auto(l4),f,15,22,r,minivan
46,dodge,caravan 2wd,3.8,1999,6,auto(l4),f,15,21,r,minivan
47,dodge,caravan 2wd,3.8,2008,6,auto(l6),f,16,23,r,minivan
48,dodge,caravan 2wd,4,2008,6,auto(l6),f,16,23,r,minivan
49,dodge,dakota pickup 4wd,3.7,2008,6,manual(m6),4,15,19,r,pickup
50,dodge,dakota pickup 4wd,3.7,2008,6,auto(l4),4,14,18,r,pickup
51,dodge,dakota pickup 4wd,3.9,1999,6,auto(l4),4,13,17,r,pickup
52,dodge,dakota pickup 4wd,3.9,1999,6,manual(m5),4,14,17,r,pickup
53,dodge,dakota pickup 4wd,4.7,2008,8,auto(l5),4,14,19,r,pickup
54,dodge,dakota pickup 4wd,4.7,2008,8,auto(l5),4,14,19,r,pickup
55,dodge,dakota pickup 4wd,4.7,2008,8,auto(l5),4,9,12,e,pickup
56,dodge,dakota pickup 4wd,5.2,1999,8,manual(m5),4,11,17,r,pickup
57,dodge,dakota pickup 4wd,5.2,1999,8,auto(l4),4,11,15,r,pickup
58,dodge,durango 4wd,3.9,1999,6,auto(l4),4,13,17,r,suv
59,dodge,durango 4wd,4.7,2008,8,auto(l5),4,13,17,r,suv
60,dodge,durango 4wd,4.7,2008,8,auto(l5),4,9,12,e,suv
61,dodge,durango 4wd,4.7,2008,8,auto(l5),4,13,17,r,suv
62,dodge,durango 4wd,5.2,1999,8,auto(l4),4,11,16,r,suv
63,dodge,durango 4wd,5.7,2008,8,auto(l5),4,13,18,r,suv
64,dodge,durango 4wd,5.9,1999,8,auto(l4),4,11,15,r,suv
65,dodge,ram 1500 pickup 4wd,4.7,2008,8,manual(m6),4,12,16,r,pickup
66,dodge,ram 1500 pickup 4wd,4.7,2008,8,auto(l5),4,9,12,e,pickup
67,dodge,ram 1500 pickup 4wd,4.7,2008,8,auto(l5),4,13,17,r,pickup
68,dodge,ram 1500 pickup 4wd,4.7,2008,8,auto(l5),4,13,17,r,pickup
69,dodge,ram 1500 pickup 4wd,4.7,2008,8,manual(m6),4,12,16,r,pickup
70,dodge,ram 1500 pickup 4wd,4.7,2008,8,manual(m6),4,9,12,e,pickup
71,dodge,ram 1500 pickup 4wd,5.2,1999,8,auto(l4),4,11,15,r,pickup
72,dodge,ram 1500 pickup 4wd,5.2,1999,8,manual(m5),4,11,16,r,pickup
73,dodge,ram 1500 pickup 4wd,5.7,2008,8,auto(l5),4,13,17,r,pickup
74,dodge,ram 1500 pickup 4wd,5.9,1999,8,auto(l4),4,11,15,r,pickup
75,ford,expedition 2wd,4.6,1999,8,auto(l4),r,11,17,r,suv
76,ford,expedition 2wd,5.4,1999,8,auto(l4),r,11,17,r,suv
77,ford,expedition 2wd,5.4,2008,8,auto(l6),r,12,18,r,suv
78,ford,explorer 4wd,4,1999,6,auto(l5),4,14,17,r,suv
79,ford,explorer 4wd,4,1999,6,manual(m5),4,15,19,r,suv
80,ford,explorer 4wd,4,1999,6,auto(l5),4,14,17,r,suv
81,ford,explorer 4wd,4,2008,6,auto(l5),4,13,19,r,suv
82,ford,explorer 4wd,4.6,2008,8,auto(l6),4,13,19,r,suv
83,ford,explorer 4wd,5,1999,8,auto(l4),4,13,17,r,suv
84,ford,f150 pickup 4wd,4.2,1999,6,auto(l4),4,14,17,r,pickup
85,ford,f150 pickup 4wd,4.2,1999,6,manual(m5),4,14,17,r,pickup
86,ford,f150 pickup 4wd,4.6,1999,8,manual(m5),4,13,16,r,pickup
87,ford,f150 pickup 4wd,4.6,1999,8,auto(l4),4,13,16,r,pickup
88,ford,f150 pickup 4wd,4.6,2008,8,auto(l4),4,13,17,r,pickup
89,ford,f150 pickup 4wd,5.4,1999,8,auto(l4),4,11,15,r,pickup
90,ford,f150 pickup 4wd,5.4,2008,8,auto(l4),4,13,17,r,pickup
91,ford,mustang,3.8,1999,6,manual(m5),r,18,26,r,subcompact
92,ford,mustang,3.8,1999,6,auto(l4),r,18,25,r,subcompact
93,ford,mustang,4,2008,6,manual(m5),r,17,26,r,subcompact
94,ford,mustang,4,2008,6,auto(l5),r,16,24,r,subcompact
95,ford,mustang,4.6,1999,8,auto(l4),r,15,21,r,subcompact
96,ford,mustang,4.6,1999,8,manual(m5),r,15,22,r,subcompact
97,ford,mustang,4.6,2008,8,manual(m5),r,15,23,r,subcompact
98,ford,mustang,4.6,2008,8,auto(l5),r,15,22,r,subcompact
99,ford,mustang,5.4,2008,8,manual(m6),r,14,20,p,subcompact
100,honda,civic,1.6,1999,4,manual(m5),f,28,33,r,subcompact
101,honda,civic,1.6,1999,4,auto(l4),f,24,32,r,subcompact
102,honda,civic,1.6,1999,4,manual(m5),f,25,32,r,subcompact
103,honda,civic,1.6,1999,4,manual(m5),f,23,29,p,subcompact
104,honda,civic,1.6,1999,4,auto(l4),f,24,32,r,subcompact
105,honda,civic,1.8,2008,4,manual(m5),f,26,34,r,subcompact
106,honda,civic,1.8,2008,4,auto(l5),f,25,36,r,subcompact
107,honda,civic,1.8,2008,4,auto(l5),f,24,36,c,subcompact
108,honda,civic,2,2008,4,manual(m6),f,21,29,p,subcompact
109,hyundai,sonata,2.4,1999,4,auto(l4),f,18,26,r,midsize
110,hyundai,sonata,2.4,1999,4,manual(m5),f,18,27,r,midsize
111,hyundai,sonata,2.4,2008,4,auto(l4),f,21,30,r,midsize
112,hyundai,sonata,2.4,2008,4,manual(m5),f,21,31,r,midsize
113,hyundai,sonata,2.5,1999,6,auto(l4),f,18,26,r,midsize
114,hyundai,sonata,2.5,1999,6,manual(m5),f,18,26,r,midsize
115,hyundai,sonata,3.3,2008,6,auto(l5),f,19,28,r,midsize
116,hyundai,tiburon,2,1999,4,auto(l4),f,19,26,r,subcompact
117,hyundai,tiburon,2,1999,4,manual(m5),f,19,29,r,subcompact
118,hyundai,tiburon,2,2008,4,manual(m5),f,20,28,r,subcompact
119,hyundai,tiburon,2,2008,4,auto(l4),f,20,27,r,subcompact
120,hyundai,tiburon,2.7,2008,6,auto(l4),f,17,24,r,subcompact
121,hyundai,tiburon,2.7,2008,6,manual(m6),f,16,24,r,subcompact
122,hyundai,tiburon,2.7,2008,6,manual(m5),f,17,24,r,subcompact
123,jeep,grand cherokee 4wd,3,2008,6,auto(l5),4,17,22,d,suv
124,jeep,grand cherokee 4wd,3.7,2008,6,auto(l5),4,15,19,r,suv
125,jeep,grand cherokee 4wd,4,1999,6,auto(l4),4,15,20,r,suv
126,jeep,grand cherokee 4wd,4.7,1999,8,auto(l4),4,14,17,r,suv
127,jeep,grand cherokee 4wd,4.7,2008,8,auto(l5),4,9,12,e,suv
128,jeep,grand cherokee 4wd,4.7,2008,8,auto(l5),4,14,19,r,suv
129,jeep,grand cherokee 4wd,5.7,2008,8,auto(l5),4,13,18,r,suv
130,jeep,grand cherokee 4wd,6.1,2008,8,auto(l5),4,11,14,p,suv
131,land rover,range rover,4,1999,8,auto(l4),4,11,15,p,suv
132,land rover,range rover,4.2,2008,8,auto(s6),4,12,18,r,suv
133,land rover,range rover,4.4,2008,8,auto(s6),4,12,18,r,suv
134,land rover,range rover,4.6,1999,8,auto(l4),4,11,15,p,suv
135,lincoln,navigator 2wd,5.4,1999,8,auto(l4),r,11,17,r,suv
136,lincoln,navigator 2wd,5.4,1999,8,auto(l4),r,11,16,p,suv
137,lincoln,navigator 2wd,5.4,2008,8,auto(l6),r,12,18,r,suv
138,mercury,mountaineer 4wd,4,1999,6,auto(l5),4,14,17,r,suv
139,mercury,mountaineer 4wd,4,2008,6,auto(l5),4,13,19,r,suv
140,mercury,mountaineer 4wd,4.6,2008,8,auto(l6),4,13,19,r,suv
141,mercury,mountaineer 4wd,5,1999,8,auto(l4),4,13,17,r,suv
142,nissan,altima,2.4,1999,4,manual(m5),f,21,29,r,compact
143,nissan,altima,2.4,1999,4,auto(l4),f,19,27,r,compact
144,nissan,altima,2.5,2008,4,auto(av),f,23,31,r,midsize
145,nissan,altima,2.5,2008,4,manual(m6),f,23,32,r,midsize
146,nissan,altima,3.5,2008,6,manual(m6),f,19,27,p,midsize
147,nissan,altima,3.5,2008,6,auto(av),f,19,26,p,midsize
148,nissan,maxima,3,1999,6,auto(l4),f,18,26,r,midsize
149,nissan,maxima,3,1999,6,manual(m5),f,19,25,r,midsize
150,nissan,maxima,3.5,2008,6,auto(av),f,19,25,p,midsize
151,nissan,pathfinder 4wd,3.3,1999,6,auto(l4),4,14,17,r,suv
152,nissan,pathfinder 4wd,3.3,1999,6,manual(m5),4,15,17,r,suv
153,nissan,pathfinder 4wd,4,2008,6,auto(l5),4,14,20,p,suv
154,nissan,pathfinder 4wd,5.6,2008,8,auto(s5),4,12,18,p,suv
155,pontiac,grand prix,3.1,1999,6,auto(l4),f,18,26,r,midsize
156,pontiac,grand prix,3.8,1999,6,auto(l4),f,16,26,p,midsize
157,pontiac,grand prix,3.8,1999,6,auto(l4),f,17,27,r,midsize
158,pontiac,grand prix,3.8,2008,6,auto(l4),f,18,28,r,midsize
159,pontiac,grand prix,5.3,2008,8,auto(s4),f,16,25,p,midsize
160,subaru,forester awd,2.5,1999,4,manual(m5),4,18,25,r,suv
161,subaru,forester awd,2.5,1999,4,auto(l4),4,18,24,r,suv
162,subaru,forester awd,2.5,2008,4,manual(m5),4,20,27,r,suv
163,subaru,forester awd,2.5,2008,4,manual(m5),4,19,25,p,suv
164,subaru,forester awd,2.5,2008,4,auto(l4),4,20,26,r,suv
165,subaru,forester awd,2.5,2008,4,auto(l4),4,18,23,p,suv
166,subaru,impreza awd,2.2,1999,4,auto(l4),4,21,26,r,subcompact
167,subaru,impreza awd,2.2,1999,4,manual(m5),4,19,26,r,subcompact
168,subaru,impreza awd,2.5,1999,4,manual(m5),4,19,26,r,subcompact
169,subaru,impreza awd,2.5,1999,4,auto(l4),4,19,26,r,subcompact
170,subaru,impreza awd,2.5,2008,4,auto(s4),4,20,25,p,compact
171,subaru,impreza awd,2.5,2008,4,auto(s4),4,20,27,r,compact
172,subaru,impreza awd,2.5,2008,4,manual(m5),4,19,25,p,compact
173,subaru,impreza awd,2.5,2008,4,manual(m5),4,20,27,r,compact
174,toyota,4runner 4wd,2.7,1999,4,manual(m5),4,15,20,r,suv
175,toyota,4runner 4wd,2.7,1999,4,auto(l4),4,16,20,r,suv
176,toyota,4runner 4wd,3.4,1999,6,auto(l4),4,15,19,r,suv
177,toyota,4runner 4wd,3.4,1999,6,manual(m5),4,15,17,r,suv
178,toyota,4runner 4wd,4,2008,6,auto(l5),4,16,20,r,suv
179,toyota,4runner 4wd,4.7,2008,8,auto(l5),4,14,17,r,suv
180,toyota,camry,2.2,1999,4,manual(m5),f,21,29,r,midsize
181,toyota,camry,2.2,1999,4,auto(l4),f,21,27,r,midsize
182,toyota,camry,2.4,2008,4,manual(m5),f,21,31,r,midsize
183,toyota,camry,2.4,2008,4,auto(l5),f,21,31,r,midsize
184,toyota,camry,3,1999,6,auto(l4),f,18,26,r,midsize
185,toyota,camry,3,1999,6,manual(m5),f,18,26,r,midsize
186,toyota,camry,3.5,2008,6,auto(s6),f,19,28,r,midsize
187,toyota,camry solara,2.2,1999,4,auto(l4),f,21,27,r,compact
188,toyota,camry solara,2.2,1999,4,manual(m5),f,21,29,r,compact
189,toyota,camry solara,2.4,2008,4,manual(m5),f,21,31,r,compact
190,toyota,camry solara,2.4,2008,4,auto(s5),f,22,31,r,compact
191,toyota,camry solara,3,1999,6,auto(l4),f,18,26,r,compact
192,toyota,camry solara,3,1999,6,manual(m5),f,18,26,r,compact
193,toyota,camry solara,3.3,2008,6,auto(s5),f,18,27,r,compact
194,toyota,corolla,1.8,1999,4,auto(l3),f,24,30,r,compact
195,toyota,corolla,1.8,1999,4,auto(l4),f,24,33,r,compact
196,toyota,corolla,1.8,1999,4,manual(m5),f,26,35,r,compact
197,toyota,corolla,1.8,2008,4,manual(m5),f,28,37,r,compact
198,toyota,corolla,1.8,2008,4,auto(l4),f,26,35,r,compact
199,toyota,land cruiser wagon 4wd,4.7,1999,8,auto(l4),4,11,15,r,suv
200,toyota,land cruiser wagon 4wd,5.7,2008,8,auto(s6),4,13,18,r,suv
201,toyota,toyota tacoma 4wd,2.7,1999,4,manual(m5),4,15,20,r,pickup
202,toyota,toyota tacoma 4wd,2.7,1999,4,auto(l4),4,16,20,r,pickup
203,toyota,toyota tacoma 4wd,2.7,2008,4,manual(m5),4,17,22,r,pickup
204,toyota,toyota tacoma 4wd,3.4,1999,6,manual(m5),4,15,17,r,pickup
205,toyota,toyota tacoma 4wd,3.4,1999,6,auto(l4),4,15,19,r,pickup
206,toyota,toyota tacoma 4wd,4,2008,6,manual(m6),4,15,18,r,pickup
207,toyota,toyota tacoma 4wd,4,2008,6,auto(l5),4,16,20,r,pickup
208,volkswagen,gti,2,1999,4,manual(m5),f,21,29,r,compact
209,volkswagen,gti,2,1999,4,auto(l4),f,19,26,r,compact
210,volkswagen,gti,2,2008,4,manual(m6),f,21,29,p,compact
211,volkswagen,gti,2,2008,4,auto(s6),f,22,29,p,compact
212,volkswagen,gti,2.8,1999,6,manual(m5),f,17,24,r,compact
213,volkswagen,jetta,1.9,1999,4,manual(m5),f,33,44,d,compact
214,volkswagen,jetta,2,1999,4,manual(m5),f,21,29,r,compact
215,volkswagen,jetta,2,1999,4,auto(l4),f,19,26,r,compact
216,volkswagen,jetta,2,2008,4,auto(s6),f,22,29,p,compact
217,volkswagen,jetta,2,2008,4,manual(m6),f,21,29,p,compact
218,volkswagen,jetta,2.5,2008,5,auto(s6),f,21,29,r,compact
219,volkswagen,jetta,2.5,2008,5,manual(m5),f,21,29,r,compact
220,volkswagen,jetta,2.8,1999,6,auto(l4),f,16,23,r,compact
221,volkswagen,jetta,2.8,1999,6,manual(m5),f,17,24,r,compact
222,volkswagen,new beetle,1.9,1999,4,manual(m5),f,35,44,d,subcompact
223,volkswagen,new beetle,1.9,1999,4,auto(l4),f,29,41,d,subcompact
224,volkswagen,new beetle,2,1999,4,manual(m5),f,21,29,r,subcompact
225,volkswagen,new beetle,2,1999,4,auto(l4),f,19,26,r,subcompact
226,volkswagen,new beetle,2.5,2008,5,manual(m5),f,20,28,r,subcompact
227,volkswagen,new beetle,2.5,2008,5,auto(s6),f,20,29,r,subcompact
228,volkswagen,passat,1.8,1999,4,manual(m5),f,21,29,p,midsize
229,volkswagen,passat,1.8,1999,4,auto(l5),f,18,29,p,midsize
230,volkswagen,passat,2,2008,4,auto(s6),f,19,28,p,midsize
231,volkswagen,passat,2,2008,4,manual(m6),f,21,29,p,midsize
232,volkswagen,passat,2.8,1999,6,auto(l5),f,16,26,p,midsize
233,volkswagen,passat,2.8,1999,6,manual(m5),f,18,26,p,midsize
234,volkswagen,passat,3.6,2008,6,auto(s6),f,17,26,p,midsize
"## },
    Dataset { name: "economics", package: "ggplot2", description: "US economic time series (23-month sample embedded, 1967-2014)", csv: r##"
rownames,date,pce,pop,psavert,uempmed,unemploy
1,1967-07-01,506.7,198712,12.6,4.5,2944
25,1969-07-01,602.7,202677,11.8,4.4,2868
49,1971-07-01,698.9,207661,13.8,6.2,5035
73,1973-07-01,854.1,211909,13.2,5.2,4305
97,1975-07-01,1039.8,215973,12.6,8.6,8127
121,1977-07-01,1280.5,220239,10.5,7.1,6829
145,1979-07-01,1586.1,225055,10.6,5.9,5996
169,1981-07-01,1942.1,229966,12.3,7.1,7863
193,1983-07-01,2304.4,234307,9.6,10.1,10548
217,1985-07-01,2715.7,238466,9.1,6.9,8513
241,1987-07-01,3094.7,242804,7.5,6.5,7268
265,1989-07-01,3585.7,247342,8.2,5.6,6495
289,1991-07-01,3966,253493,8.2,7,8586
313,1993-07-01,4468.9,260255,7.6,8.2,8930
337,1995-07-01,4970.2,266557,7.1,8.5,7527
361,1997-07-01,5548.8,272912,6.1,8.3,6655
385,1999-07-01,6281.5,279295,4.8,5.8,6025
409,2001-07-01,7072.2,285309,5.6,6.8,6583
433,2003-07-01,7738.2,290820,6.3,10.3,9011
457,2005-07-01,8829.5,296186,2.2,8.8,7406
481,2007-07-01,9709.6,302004,3.7,8.7,7149
505,2009-07-01,9841.7,307439,6,16,14601
529,2011-07-01,10677.5,311997.049,7.3,22,13763
"## },
    Dataset { name: "msleep", package: "ggplot2", description: "Sleep times of 83 mammals (full dataset)", csv: r##"
rownames,name,genus,vore,order,conservation,sleep_total,sleep_rem,sleep_cycle,awake,brainwt,bodywt
1,Cheetah,Acinonyx,carni,Carnivora,lc,12.1,,,11.9,,50
2,Owl monkey,Aotus,omni,Primates,,17,1.8,,7,0.0155,0.48
3,Mountain beaver,Aplodontia,herbi,Rodentia,nt,14.4,2.4,,9.6,,1.35
4,Greater short-tailed shrew,Blarina,omni,Soricomorpha,lc,14.9,2.3,0.133333333,9.1,2.9e-4,0.019
5,Cow,Bos,herbi,Artiodactyla,domesticated,4,0.7,0.666666667,20,0.423,600
6,Three-toed sloth,Bradypus,herbi,Pilosa,,14.4,2.2,0.766666667,9.6,,3.85
7,Northern fur seal,Callorhinus,carni,Carnivora,vu,8.7,1.4,0.383333333,15.3,,20.49
8,Vesper mouse,Calomys,,Rodentia,,7,,,17,,0.045
9,Dog,Canis,carni,Carnivora,domesticated,10.1,2.9,0.333333333,13.9,0.07,14
10,Roe deer,Capreolus,herbi,Artiodactyla,lc,3,,,21,0.0982,14.8
11,Goat,Capri,herbi,Artiodactyla,lc,5.3,0.6,,18.7,0.115,33.5
12,Guinea pig,Cavis,herbi,Rodentia,domesticated,9.4,0.8,0.216666667,14.6,0.0055,0.728
13,Grivet,Cercopithecus,omni,Primates,lc,10,0.7,,14,,4.75
14,Chinchilla,Chinchilla,herbi,Rodentia,domesticated,12.5,1.5,0.116666667,11.5,0.0064,0.42
15,Star-nosed mole,Condylura,omni,Soricomorpha,lc,10.3,2.2,,13.7,0.001,0.06
16,African giant pouched rat,Cricetomys,omni,Rodentia,,8.3,2,,15.7,0.0066,1
17,Lesser short-tailed shrew,Cryptotis,omni,Soricomorpha,lc,9.1,1.4,0.15,14.9,1.4e-4,0.005
18,Long-nosed armadillo,Dasypus,carni,Cingulata,lc,17.4,3.1,0.383333333,6.6,0.0108,3.5
19,Tree hyrax,Dendrohyrax,herbi,Hyracoidea,lc,5.3,0.5,,18.7,0.0123,2.95
20,North American Opossum,Didelphis,omni,Didelphimorphia,lc,18,4.9,0.333333333,6,0.0063,1.7
21,Asian elephant,Elephas,herbi,Proboscidea,en,3.9,,,20.1,4.603,2547
22,Big brown bat,Eptesicus,insecti,Chiroptera,lc,19.7,3.9,0.116666667,4.3,3e-4,0.023
23,Horse,Equus,herbi,Perissodactyla,domesticated,2.9,0.6,1,21.1,0.655,521
24,Donkey,Equus,herbi,Perissodactyla,domesticated,3.1,0.4,,20.9,0.419,187
25,European hedgehog,Erinaceus,omni,Erinaceomorpha,lc,10.1,3.5,0.283333333,13.9,0.0035,0.77
26,Patas monkey,Erythrocebus,omni,Primates,lc,10.9,1.1,,13.1,0.115,10
27,Western american chipmunk,Eutamias,herbi,Rodentia,,14.9,,,9.1,,0.071
28,Domestic cat,Felis,carni,Carnivora,domesticated,12.5,3.2,0.416666667,11.5,0.0256,3.3
29,Galago,Galago,omni,Primates,,9.8,1.1,0.55,14.2,0.005,0.2
30,Giraffe,Giraffa,herbi,Artiodactyla,cd,1.9,0.4,,22.1,,899.995
31,Pilot whale,Globicephalus,carni,Cetacea,cd,2.7,0.1,,21.35,,800
32,Gray seal,Haliochoerus,carni,Carnivora,lc,6.2,1.5,,17.8,0.325,85
33,Gray hyrax,Heterohyrax,herbi,Hyracoidea,lc,6.3,0.6,,17.7,0.01227,2.625
34,Human,Homo,omni,Primates,,8,1.9,1.5,16,1.32,62
35,Mongoose lemur,Lemur,herbi,Primates,vu,9.5,0.9,,14.5,,1.67
36,African elephant,Loxodonta,herbi,Proboscidea,vu,3.3,,,20.7,5.712,6654
37,Thick-tailed opposum,Lutreolina,carni,Didelphimorphia,lc,19.4,6.6,,4.6,,0.37
38,Macaque,Macaca,omni,Primates,,10.1,1.2,0.75,13.9,0.179,6.8
39,Mongolian gerbil,Meriones,herbi,Rodentia,lc,14.2,1.9,,9.8,,0.053
40,Golden hamster,Mesocricetus,herbi,Rodentia,en,14.3,3.1,0.2,9.7,0.001,0.12
41,Vole ,Microtus,herbi,Rodentia,,12.8,,,11.2,,0.035
42,House mouse,Mus,herbi,Rodentia,nt,12.5,1.4,0.183333333,11.5,4e-4,0.022
43,Little brown bat,Myotis,insecti,Chiroptera,,19.9,2,0.2,4.1,2.5e-4,0.01
44,Round-tailed muskrat,Neofiber,herbi,Rodentia,nt,14.6,,,9.4,,0.266
45,Slow loris,Nyctibeus,carni,Primates,,11,,,13,0.0125,1.4
46,Degu,Octodon,herbi,Rodentia,lc,7.7,0.9,,16.3,,0.21
47,Northern grasshopper mouse,Onychomys,carni,Rodentia,lc,14.5,,,9.5,,0.028
48,Rabbit,Oryctolagus,herbi,Lagomorpha,domesticated,8.4,0.9,0.416666667,15.6,0.0121,2.5
49,Sheep,Ovis,herbi,Artiodactyla,domesticated,3.8,0.6,,20.2,0.175,55.5
50,Chimpanzee,Pan,omni,Primates,,9.7,1.4,1.416666667,14.3,0.44,52.2
51,Tiger,Panthera,carni,Carnivora,en,15.8,,,8.2,,162.564
52,Jaguar,Panthera,carni,Carnivora,nt,10.4,,,13.6,0.157,100
53,Lion,Panthera,carni,Carnivora,vu,13.5,,,10.5,,161.499
54,Baboon,Papio,omni,Primates,,9.4,1,0.666666667,14.6,0.18,25.235
55,Desert hedgehog,Paraechinus,,Erinaceomorpha,lc,10.3,2.7,,13.7,0.0024,0.55
56,Potto,Perodicticus,omni,Primates,lc,11,,,13,,1.1
57,Deer mouse,Peromyscus,,Rodentia,,11.5,,,12.5,,0.021
58,Phalanger,Phalanger,,Diprotodontia,,13.7,1.8,,10.3,0.0114,1.62
59,Caspian seal,Phoca,carni,Carnivora,vu,3.5,0.4,,20.5,,86
60,Common porpoise,Phocoena,carni,Cetacea,vu,5.6,,,18.45,,53.18
61,Potoroo,Potorous,herbi,Diprotodontia,,11.1,1.5,,12.9,,1.1
62,Giant armadillo,Priodontes,insecti,Cingulata,en,18.1,6.1,,5.9,0.081,60
63,Rock hyrax,Procavia,,Hyracoidea,lc,5.4,0.5,,18.6,0.021,3.6
64,Laboratory rat,Rattus,herbi,Rodentia,lc,13,2.4,0.183333333,11,0.0019,0.32
65,African striped mouse,Rhabdomys,omni,Rodentia,,8.7,,,15.3,,0.044
66,Squirrel monkey,Saimiri,omni,Primates,,9.6,1.4,,14.4,0.02,0.743
67,Eastern american mole,Scalopus,insecti,Soricomorpha,lc,8.4,2.1,0.166666667,15.6,0.0012,0.075
68,Cotton rat,Sigmodon,herbi,Rodentia,,11.3,1.1,0.15,12.7,0.00118,0.148
69,Mole rat,Spalax,,Rodentia,,10.6,2.4,,13.4,0.003,0.122
70,Arctic ground squirrel,Spermophilus,herbi,Rodentia,lc,16.6,,,7.4,0.0057,0.92
71,Thirteen-lined ground squirrel,Spermophilus,herbi,Rodentia,lc,13.8,3.4,0.216666667,10.2,0.004,0.101
72,Golden-mantled ground squirrel,Spermophilus,herbi,Rodentia,lc,15.9,3,,8.1,,0.205
73,Musk shrew,Suncus,,Soricomorpha,,12.8,2,0.183333333,11.2,3.3e-4,0.048
74,Pig,Sus,omni,Artiodactyla,domesticated,9.1,2.4,0.5,14.9,0.18,86.25
75,Short-nosed echidna,Tachyglossus,insecti,Monotremata,,8.6,,,15.4,0.025,4.5
76,Eastern american chipmunk,Tamias,herbi,Rodentia,,15.8,,,8.2,,0.112
77,Brazilian tapir,Tapirus,herbi,Perissodactyla,vu,4.4,1,0.9,19.6,0.169,207.501
78,Tenrec,Tenrec,omni,Afrosoricida,,15.6,2.3,,8.4,0.0026,0.9
79,Tree shrew,Tupaia,omni,Scandentia,,8.9,2.6,0.233333333,15.1,0.0025,0.104
80,Bottle-nosed dolphin,Tursiops,carni,Cetacea,,5.2,,,18.8,,173.33
81,Genet,Genetta,carni,Carnivora,,6.3,1.3,,17.7,0.0175,2
82,Arctic fox,Vulpes,carni,Carnivora,,12.5,,,11.5,0.0445,3.38
83,Red fox,Vulpes,carni,Carnivora,,9.8,2.4,0.35,14.2,0.0504,4.23
"## },
    Dataset { name: "midwest", package: "ggplot2", description: "Midwest demographics (12 highest-poverty counties embedded)", csv: r##"
rownames,PID,county,state,area,poptotal,popdensity,popwhite,popblack,popamerindian,popasian,popother,percwhite,percblack,percamerindan,percasian,percother,popadults,perchsd,percollege,percprof,poppovertyknown,percpovertyknown,percbelowpoverty,percchildbelowpovert,percadultpoverty,percelderlypoverty,inmetro,category
2,562,ALEXANDER,IL,0.014,10626,759,7054,3496,19,48,9,66.3843403,32.9004329,0.1788067,0.45172219,0.08469791,6724,59.7263534,11.2433076,2.87031529,10529,99.0871447,32.2442777,45.8265139,27.3856467,25.2289759,0,LHR
35,595,HARDIN,IL,0.009,5189,576.555556,5062,85,16,13,13,97.5525149,1.63808056,0.30834458,0.25052997,0.25052997,3492,59.7365407,14.3184422,2.60595647,4954,95.4711891,26.7460638,35.1417004,25.1742835,21.1961302,0,LHR
39,599,JACKSON,IL,0.036,61067,1696.30556,51991,6342,109,2178,447,85.1376357,10.3853145,0.17849248,3.56657442,0.7319829,32172,78.767251,36.6436653,14.0898918,54230,88.8041004,28.37175,26.3922105,32.4584833,13.8153008,0,AHR
76,636,POPE,IL,0.022,4373,198.772727,4072,266,15,6,14,93.1168534,6.0827807,0.34301395,0.13720558,0.32014635,2821,65.1542006,14.2502659,2.48138958,4160,95.1292019,25.1682692,36.4729459,24.4148692,15.3611394,0,LHR
77,637,PULASKI,IL,0.011,7523,683.909091,5032,2466,8,7,10,66.8882095,32.7794763,0.10634056,0.09304799,0.13292569,4816,59.7799003,14.3687708,2.20099668,7441,98.9100093,30.184115,40.0374181,23.867741,31.1619718,0,LHR
237,1239,LAKE,MI,0.035,8583,245.228571,7337,1146,81,9,10,85.4829314,13.3519748,0.94372597,0.10485844,0.11650938,5931,61.288147,11.3134379,2.00640701,8387,97.7164162,26.4099201,37.6986038,25.0707123,18.8764973,0,LHR
278,2009,ADAMS,OH,0.035,25371,724.885714,25212,47,67,30,15,99.3733002,0.18525088,0.26408104,0.11824524,0.05912262,15569,58.3852527,8.74173036,2.15813475,25028,98.6480627,28.5280486,35.0246846,25.7522586,26.1800599,0,LHR
282,2013,ATHENS,OH,0.03,59549,1984.96667,56163,1678,167,1374,167,94.3139263,2.81784749,0.28044132,2.30734353,0.28044132,30179,74.6114848,29.1494085,12.0447994,51002,85.6471141,28.6733854,29.2581157,31.7442813,14.3249554,0,AHR
330,2061,MEIGS,OH,0.025,22987,919.48,22734,177,44,20,12,98.8993779,0.77000044,0.19141254,0.0870057,0.05220342,14772,64.0265367,11.677498,2.38288654,22665,98.5992082,26.0092654,34.9729464,23.6672906,19.9579341,0,LHR
343,2074,PIKE,OH,0.026,24249,932.653846,23807,327,72,41,2,98.1772444,1.34850922,0.29691946,0.16907914,0.00824776,15099,60.7722366,12.4445328,3.23862507,23830,98.2720937,26.5757449,36.915078,22.2239587,22.7062546,0,LHR
350,2081,SCIOTO,OH,0.035,80327,2295.05714,77253,2458,409,126,81,96.1731423,3.05999228,0.50916877,0.15685884,0.10083783,51585,63.7569061,13.7772608,3.17534167,76736,95.5295231,25.7923269,37.6523745,23.5231677,15.1068091,0,LHR
405,3020,MENOMINEE,WI,0.021,3890,185.238095,416,0,3469,0,5,10.6940874,0,89.1773779,0,0.1285347,1922,62.6951093,7.33610822,0.52029136,3820,98.2005141,48.6910995,64.3084767,43.3124644,18.2186235,0,LHR
"## },
    Dataset { name: "presidential", package: "ggplot2", description: "US presidential terms 1953-2021 (12 presidents)", csv: r##"
rownames,name,start,end,party
1,Eisenhower,1953-01-20,1961-01-20,Republican
2,Kennedy,1961-01-20,1963-11-22,Democratic
3,Johnson,1963-11-22,1969-01-20,Democratic
4,Nixon,1969-01-20,1974-08-09,Republican
5,Ford,1974-08-09,1977-01-20,Republican
6,Carter,1977-01-20,1981-01-20,Democratic
7,Reagan,1981-01-20,1989-01-20,Republican
8,Bush,1989-01-20,1993-01-20,Republican
9,Clinton,1993-01-20,2001-01-20,Democratic
10,Bush,2001-01-20,2009-01-20,Republican
11,Obama,2009-01-20,2017-01-20,Democratic
12,Trump,2017-01-20,2021-01-20,Republican
"## },
];

/// A parsed CSV table (header + rows).
#[derive(Debug, Clone)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    /// Parse CSV text (simple RFC-4180-ish parser: commas, `"` quotes, no embedded newlines).
    pub fn parse(csv: &str) -> Table {
        let mut columns = Vec::new();
        let mut rows = Vec::new();
        for line in csv.lines() {
            if line.trim().is_empty() { continue; }
            let fields = parse_csv_line(line);
            if columns.is_empty() { columns = fields; } else { rows.push(fields); }
        }
        Table { columns, rows }
    }

    pub fn len(&self) -> usize { self.rows.len() }
    pub fn is_empty(&self) -> bool { self.rows.is_empty() }

    /// Column index by header name.
    pub fn column(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c == name)
    }

    /// String values of a column (empty string if the column is missing).
    pub fn col_str(&self, name: &str) -> Vec<&str> {
        match self.column(name) {
            Some(ci) => self.rows.iter().map(|r| r.get(ci).map(|s| s.as_str()).unwrap_or("")).collect(),
            None => self.rows.iter().map(|_| "").collect(),
        }
    }

    /// Numeric values of a column; unparseable/empty cells become `None`.
    pub fn col_f64(&self, name: &str) -> Vec<Option<f64>> {
        self.col_str(name).iter().map(|s| s.parse::<f64>().ok()).collect()
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') { chars.next(); cur.push('"'); } else { in_quotes = false; }
            } else { cur.push(c); }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else { cur.push(c); }
    }
    fields.push(cur);
    fields
}
