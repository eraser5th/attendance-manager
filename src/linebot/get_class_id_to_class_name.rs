use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_class_id_to_class_name() -> io::Result<HashMap<String, String>> {
    //let file = File::open("./src/syllabus.txt")?;
    let text = "HS01 哲学
    HS05 文学
    HS06 芸術学
    HS07 ジェンダー・セクシュアリティ論
    HS10 経済学
    HS11 社会学
    HS12 日本国憲法
    HS13 国際関係論
    HS16 保健学
    HS17 科学史
    HS19 会津の歴史と文化
    HS20 アカデミックスキル１
    HS21 アカデミックスキル２
    HS22 地域社会学
    HS23 経済発展論
    HS24 ビジネス・コミュニケーション
    HS25 経営戦略論
    HS26 ベンチャービジネス論
    HS27 心理言語学概論
    SS01 健康・スポーツ科学実習1
    SS02 健康・スポーツ科学実習2
    SS03 健康・スポーツ科学実習3
    SS04 生涯スポーツ科学実習
    EN01 Introductory English 1
    EN02 Introductory English 2
    EN03 Bridge 1 to Intermediate English
    EN04 Bridge 2 to Intermediate English
    EN05 Intermediate English 1
    EN06 Intermediate English 2
    EN07 Advanced English
    EN08 Thesis Writing and Presentation
    EG101 Global Experience Gateway (RHIT)
    EG102 Global Experience Gateway (Waikato)
    JP01 初級日本語 I
    JP02 初級日本語 II
    JP03 中級日本語 I
    JP04 中級日本語 II
    JP05 上級日本語 I
    JP06 上級日本語 II
    JP07 ビジネス日本語
    EL102 Design of Human Languages
    EL115 Analysis of English Sentence Structure
    EL131 Language and Linguistics
    EL134 High Frequency Vocabulary
    EL144 Conversation Analysis and the Pragmatics of Spoken Interaction
    EL146 Corpus Linguistics for Language Learners
    EL152 Reading Fluency
    EL154 SDGs で学ぶ英語ディスカッション
    EL155 Notetaking and Listening Skills for Academic Lectures in English
    EL156 Sociolinguistics: Language and Society
    EL157 Networking Skills in International Industry Environment
    EL160 Language Education in the Age of AI
    EL218 English Speaking and Presentation Skills
    EL244 An Introduction to Cross-cultural Communication
    EL248 Visualization and Storytelling in Data Science
    EL314 Experimental Methods and Statistics for Linguistics
    EL315 Content Management for IT Business
    EL317 Patterns and language
    EL318 ICT in Education
    EL321 Pronunciation: Acoustic Analysis Using Software
    EL329 Critical Thinking
    EL330 Computer Science Vocabulary
    EL331 Authorship Analysis using Python
    MA01 線形代数 I
    MA02 線形代数 II
    MA03 微積分 I
    MA04 微積分 II
    MA05 フーリエ解析
    MA06 複素関数論
    MA07 確率統計学
    MA08 応用代数
    MA09 数理論理学
    MA10 位相幾何学概論
    MA11 応用幾何とトポロジー
    NS01 力学
    NS02 電磁気学
    NS03 量子力学
    NS04 半導体デバイス
    NS05 熱・統計力学
    LI01 コンピュータリテラシー
    LI03 コンピュータ理工学のすすめ
    LI04 コンピュータシステム概論
    LI06 情報セキュリティ
    LI07 情報と職業
    LI08 情報倫理
    LI09 システム開発とプロジェクトマネジメントの基礎
    LI10 マルチメディアシステム概論
    LI11 コンピュータネットワーク概論
    LI12 創造力開発スタジオ
    LI13 コンピュータ理工学演習 I
    LI14 コンピュータ理工学演習 II
    PL01 プログラミング入門
    PL02 プログラミングC
    PL03 プログラミングJAVA I
    PL04 プログラミングC++
    PL05 コンピュータ言語論
    PL06 プログラミングJAVA II
    FU01 アルゴリズムとデータ構造 I
    FU02 情報理論と圧縮
    FU03 離散系論
    FU04 論理回路設計論
    FU05 コンピュータアーキテクチャ論
    FU06 オペレーティングシステム論
    FU08 オートマトンと言語理論
    FU09 アルゴリズムとデータ構造 II
    FU10 言語処理系論
    FU11 数値解析
    FU14 ソフトウェア工学概論
    FU15 データマネジメント概論
    SY02 電子回路
    SY04 組込みシステム
    SY05 並列コンピュータシステム
    SY06 VLSI設計技術
    SY07 論理回路設計特論
    CN02 ネットワークセキュリティ
    CN03 ネットワークプログラミング
    CN04 ワイヤレスネットワーク
    CN05 コンピュータネットワークシステムのモデリングとシミュレーション
    IT01 人工知能
    IT02 コンピュータグラフィックス論
    IT03 画像処理論
    IT05 ロボット工学と自動制御
    IT06 ヒューマンインターフェイスと仮想現実
    IT08 信号処理と線形システム
    IT09 音響音声処理論
    IT10 ビジュアルコンピューティングのための幾何学
    IT11 情報検索と自然言語処理
    SE01 ウェブエンジニアリング
    SE04 ソフトウェア工学特論
    SE05 ソフトウェアスタジオ
    SE06 並列分散コンピューティング
    SE07 Pythonによるデータサイエンス概論
    SE08 ビッグデータ分析概論
    OT01-I ベンチャー基本コース各論 I
    OT01-II ベンチャー基本コース各論 II
    OT02-1 ベンチャー体験工房 1
    OT02-2 ベンチャー体験工房 2
    OT02-3 ベンチャー体験工房 3
    OT02-5 ベンチャー体験工房 5
    OT02-6 ベンチャー体験工房 6
    OT02-9 ベンチャー体験工房 9
    OT04 情報処理試験対策講座
    OT05 キャリアデザインI
    OT06 キャリアデザインII
    OT08 TOEIC準備コース(Level A)
    OT08 TOEIC準備コース(Level B)
    OT10 課外活動コース II＜インターンシップIII（大連）＞
    OT10 課外活動コースII＜インターンシップII（国内A）＞
    OT10 課外活動コースII＜インターンシップII（国内C）＞
    OT10 課外活動コースII＜インターンシップII（国内D）＞
    OT10 課外活動コースII＜インターンシップI（地域A）＞
    OT10 課外活動コース II＜インターンシップIII（シリコンバレーA）＞
    OT11 ICTベンチャー起業と経営
    TE01 教師入門
    TE02 教育入門
    TE03 教育心理学
    TE04 教育課程論
    TE05 教育方法
    TE06 数学科教育法１
    TE07 数学科教育法２
    TE08 数学科教育法３
    TE09 数学科教育法４
    TE10 情報科教育法１
    TE11 情報科教育法２
    TE12 道徳教育
    TE13 特別活動
    TE14 生徒指導・教育相談
    TE15 キャリア教育
    TE16 教育実習１
    TE17 教育実習２
    TE18 教育実習事前事後指導
    TE19 教育制度論
    TE20 教職実践演習（中・高）
    TE21 特別支援教育入門
    TE22 総合的な学習の時間の指導法
    TE23 情報機器の活用に関する理論と方法
    IE01 システム総合演習 I
    IE02 システム総合演習 II
    IE03 ソフトウェア総合演習 I
    IE04 ソフトウェア総合演習 II
    ";
    let reader = BufReader::new(text.as_bytes());

    let mut hash_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let key = parts[0].to_string();
            let value = parts[1..].join(" ");
            hash_map.insert(key, value);
        }
    }

    Ok(hash_map)
}
