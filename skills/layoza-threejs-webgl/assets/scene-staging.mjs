// MIT. Framework-neutral camera fit and semantic phases, with injected THREE.
// Pass the actual model bounds AFTER applying the current pose. Exclude infinite
// floors, lights, HTML labels and decorative backdrops from those bounds.
export function fitPerspectiveBox(THREE, camera, box, {direction=new THREE.Vector3(.64,.56,.76),padding=1.12}={}) {
  if(box.isEmpty()||!Number.isFinite(camera.aspect)||camera.aspect<=0)return;
  const center=box.getCenter(new THREE.Vector3()),view=direction.clone().normalize();
  const right=new THREE.Vector3().crossVectors(camera.up,view).normalize(),up=new THREE.Vector3().crossVectors(view,right).normalize();
  const halfFov=Math.tan(THREE.MathUtils.degToRad(camera.fov*.5));let distance=camera.near*3;
  for(const x of[box.min.x,box.max.x])for(const y of[box.min.y,box.max.y])for(const z of[box.min.z,box.max.z]){
    const v=new THREE.Vector3(x,y,z).sub(center),depth=v.dot(view);
    distance=Math.max(distance,depth+Math.abs(v.dot(right))*padding/(halfFov*camera.aspect),depth+Math.abs(v.dot(up))*padding/halfFov);
  }
  camera.position.copy(center).addScaledVector(view,distance);camera.lookAt(center);camera.updateMatrixWorld();camera.updateProjectionMatrix();
}

// Phase thresholds must follow ACTUAL geometry transitions, not equal thirds by
// habit. A caller supplies its own subject, labels and explanatory copy.
export function scenePhase(progress,phases){
  if(!Array.isArray(phases)||!phases.length)throw new Error('Supply the scene’s actual phases');
  let selected=phases[0];for(const phase of phases){if(progress>=phase.start)selected=phase;}
  return selected;
}
