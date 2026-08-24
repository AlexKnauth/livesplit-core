use crate::{
    Run,
    auto_splitting::settings::{
        List as AutoSplitterSettingsList, Value as AutoSplitterSettingValue,
    },
};

#[test]
fn set_legacy_raw_xml() {
    let xml = r#"
        <Ordered>True</Ordered>
        <AutosplitEndRuns>True</AutosplitEndRuns>
        <AutosplitStartRuns></AutosplitStartRuns>
        <Splits>
            <Split>VengefulSpirit</Split>
            <Split>EnterGreenpath</Split>
            <Split>MothwingCloak</Split>
            <Split>Aluba</Split>
        </Splits>
    "#;

    let mut run = Run::new();
    run.auto_splitter_settings = xml.to_string();

    let s = run.stored_auto_splitter_settings();
    assert_eq!(s.is_ok(), true);
    let mut s = s.expect("should be ok");
    assert_eq!(s.is_legacy_raw_xml(), true);
    assert_eq!(s.settings_map().len(), 1);
    assert_eq!(
        s.settings_map().get("legacy_raw_xml"),
        Some(&AutoSplitterSettingValue::String(xml.trim().into()))
    );
    assert_eq!(run.auto_splitter_settings(), xml);

    s.settings_map_mut()
        .insert("Ordered".into(), AutoSplitterSettingValue::Bool(true));
    s.settings_map_mut().insert(
        "AutosplitEndRuns".into(),
        AutoSplitterSettingValue::Bool(true),
    );
    s.settings_map_mut().insert(
        "AutosplitStartRuns".into(),
        AutoSplitterSettingValue::String("".into()),
    );
    let mut splits = AutoSplitterSettingsList::new();
    splits.push(AutoSplitterSettingValue::String("VengefulSpirit".into()));
    splits.push(AutoSplitterSettingValue::String("EnterGreenpath".into()));
    splits.push(AutoSplitterSettingValue::String("MothwingCloak".into()));
    splits.push(AutoSplitterSettingValue::String("Aluba".into()));
    s.settings_map_mut()
        .insert("Splits".into(), AutoSplitterSettingValue::List(splits));

    run.set_stored_auto_splitter_settings(&s);
    assert_eq!(run.auto_splitter_settings(), xml);

    s.settings_map_mut().remove("legacy_raw_xml");

    run.set_stored_auto_splitter_settings(&s);
    assert_ne!(run.auto_splitter_settings(), xml);
    assert!(
        run.auto_splitter_settings()
            .contains("<Version>1.0</Version>")
    );
    assert!(run.auto_splitter_settings().contains("<CustomSettings>"));
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting id=\"Splits\" type=\"list\">")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting type=\"string\" value=\"VengefulSpirit\"/>")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting type=\"string\" value=\"EnterGreenpath\"/>")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting type=\"string\" value=\"MothwingCloak\"/>")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting type=\"string\" value=\"Aluba\"/>")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting id=\"Ordered\" type=\"bool\">True</Setting>")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting id=\"AutosplitEndRuns\" type=\"bool\">True</Setting>")
    );
    assert!(
        run.auto_splitter_settings()
            .contains("<Setting id=\"AutosplitStartRuns\" type=\"string\" value=\"\"/>")
    );
    assert!(run.auto_splitter_settings().contains("</CustomSettings>"));
    let s2 = run.stored_auto_splitter_settings();
    assert_eq!(s2.is_ok(), true);
    let s2 = s2.expect("should be ok");
    assert_eq!(s2.is_legacy_raw_xml(), false);
    assert_eq!(s2.settings_map().len(), 4);
}
