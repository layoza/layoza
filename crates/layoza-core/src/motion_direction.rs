//! Authored section studies: explicit client content, candidate mechanisms, no fake novelty score.
use serde::{Deserialize,Serialize};
use serde_json::{json,Value};

#[derive(Debug,Deserialize,Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotionPlan { pub thesis:String, pub sections:Vec<MotionSection> }
#[derive(Debug,Deserialize,Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotionSection {
    pub id:String, pub role:String, pub object:String, pub visitor_job:String,
    #[serde(default)] pub family:Option<String>,
    #[serde(default)] pub transformation:Option<String>,
    #[serde(default)] pub resolved_state:Option<String>,
    #[serde(default)] pub phone:Option<String>,
}
pub const FAMILIES:&[&str]=&["material_assembly","editorial_recomposition","shared_element_handoff","media_aperture","perspective_sequence","spatial_diagram","path_progression","interactive_simulation","state_continuity","held_reading"];
pub fn study(plan:Option<&Value>,dial:u8)->Result<Value,String>{
    if dial==0{return Ok(json!({"status":"static","notice":"Motion is explicitly off. Keep readable source content and immediate semantic/focus feedback. No animation library, decorative timeline or motion plan is required.","runtime_asset":null,"review_command":null}))}
    let Some(value)=plan else{return Ok(json!({"status":"not_authored","notice":"A recipe or motion dial does not decide choreography. Supply motion_plan with a subject-specific thesis and sections; author the transformations, then inspect actual scroll playback.","families":FAMILIES,"runtime_asset":"skills/layoza-animations/assets/section-motion.mjs","review_command":"node scripts/review-motion.mjs <URL> --out <directory>","section_fields":["id","role","object","visitor_job","family","transformation","resolved_state","phone"]}))};
    let plan:MotionPlan=serde_json::from_value(value.clone()).map_err(|e|format!("Invalid motion_plan: {e}"))?;
    if plan.thesis.trim().is_empty()||plan.thesis.len()>4000||plan.sections.is_empty()||plan.sections.len()>32{return Err("motion_plan needs a real thesis and 1-32 sections.".into())}
    let mut ids=std::collections::HashSet::new();let mut warnings=Vec::new();let mut assigned=0;let mut rows=Vec::new();
    for s in &plan.sections{
        if [&s.id,&s.role,&s.object,&s.visitor_job].iter().any(|v|v.trim().is_empty()||v.len()>4000)||!ids.insert(&s.id){return Err("Motion sections need unique ids and nonempty role, object and visitor_job (max 4000 characters each).".into())}
        if let Some(f)=&s.family{if !FAMILIES.contains(&f.as_str()){return Err(format!("Unknown motion family `{f}`. Supported: {}",FAMILIES.join(", ")))} }
        let candidates:&[&str]=match s.role.as_str(){
            "portfolio"=>&["media_aperture","shared_element_handoff","perspective_sequence"],
            "services"|"expertise"=>&["material_assembly","spatial_diagram","state_continuity"],
            "process"=>&["path_progression","shared_element_handoff","spatial_diagram"],
            "playground"=>&["interactive_simulation","material_assembly"],
            "statement"|"journal"=>&["editorial_recomposition","media_aperture","held_reading"],
            "contact"=>&["shared_element_handoff","material_assembly","state_continuity"],
            "proof"|"reading"=>&["held_reading","state_continuity"],
            _=>&["editorial_recomposition","perspective_sequence","material_assembly","held_reading"],
        };
        let filled=|v:&Option<String>|v.as_ref().is_some_and(|s|!s.trim().is_empty());
        let missing:Vec<&str>=[("family",&s.family),("transformation",&s.transformation),("resolved_state",&s.resolved_state),("phone",&s.phone)].into_iter().filter(|(_,v)|!filled(v)).map(|(k,_)|k).collect();
        if missing.is_empty(){assigned+=1}
        else if missing==["family"]{warnings.push(format!("{}: the scene is authored; name its family from candidate_families ({}) so repetition across sections can be checked.",s.id,candidates.join(", ")))}
        else{warnings.push(format!("{}: missing {}. Author the actual transformation, resolved state and phone staging; a family name is not a scene.",s.id,missing.join(", ")))}
        rows.push(json!({"section":s,"candidate_families":candidates,"selection":"Author from the actual object and visitor job; candidates are ingredients, not a forced layout or a uniqueness claim.","delivery":if dial==0{"static; no decorative timelines"}else{"GSAP-owned choreography; preserve a complete readable still"}}));
    }
    if dial>=7&&plan.sections.len()>2{let meaningful:Vec<_>=plan.sections.iter().filter_map(|s|s.family.as_deref()).filter(|f|*f!="held_reading").collect();if !meaningful.is_empty()&&meaningful.iter().all(|f|*f==meaningful[0]){warnings.push("Every expressive section uses the same family. Inspect whether unrelated content is receiving a repeated preset; repetition inside one coherent gallery can be appropriate.".into())}}
    Ok(json!({"status":if assigned==plan.sections.len(){"authored_unverified"}else{"incomplete"},"thesis":plan.thesis,"sections":rows,"warnings":warnings,"runtime_asset":"skills/layoza-animations/assets/section-motion.mjs","review_command":"node scripts/review-motion.mjs <URL> --out <directory>","quality_verdict":"Not established. An authored map, many families or successful tests do not prove beauty or superiority; inspect normal desktop/phone playback and resting frames."}))
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn does_not_invent_a_completed_motion_direction(){assert_eq!(study(None,9).unwrap()["status"],"not_authored");assert!(study(Some(&json!({"thesis":"x","sections":[]})),9).is_err());let p=json!({"thesis":"Parts join into a usable system","sections":[{"id":"services","role":"services","object":"actual service layers","visitor_job":"understand the offer"}]});let r=study(Some(&p),9).unwrap();assert_eq!(r["status"],"incomplete");assert!(r["sections"][0]["candidate_families"].as_array().unwrap().len()>1);}
 #[test]fn authored_is_not_verified_and_static_stays_static(){let p=json!({"thesis":"A connected system","sections":[{"id":"proof","role":"proof","object":"real quotations","visitor_job":"read evidence","family":"held_reading","transformation":"Hold readable evidence","resolved_state":"Complete readable quote","phone":"Natural vertical reading"}]});let r=study(Some(&p),8).unwrap();assert_eq!(r["status"],"authored_unverified");assert_eq!(study(None,0).unwrap()["status"],"static");assert!(study(None,0).unwrap()["runtime_asset"].is_null());let mut duplicate=p.clone();duplicate["sections"].as_array_mut().unwrap().push(p["sections"][0].clone());assert!(study(Some(&duplicate),8).is_err());}
}
