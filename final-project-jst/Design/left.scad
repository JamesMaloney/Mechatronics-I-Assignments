length = 90;
width = 43;
height = 68;

union(){
difference(){
    translate([-((length+20)/4),-(width+10)/4,(height)/2+11]) cube([(length-10)/2,(width+30)/2,20],center=true);
    translate([0,(width)/2,0]) cube([length+25,width,height+50],center = true);
    translate([-((length+20)/4),0,(height)/2+11]) rotate ([90,0,0]) cylinder (h = 25, r=5, center = true, $fn=100);
}

difference(){
    cube([length+5,width+20,height+5],center = true);
    cube([length,width,height],center = true);
    translate([0,0,-17.5]) rotate([0,90,0]) cylinder(h=height,r=4);
    translate([0,(width)/2,0]) cube([length+25,width,height+50],center = true,$fn=100);
    translate([45,0,14]) cube(24,center = true);
    translate([0,0,-20]) cube([30,18,height],center=true);
    translate([0,0,20]) rotate ([0,0,90]) cylinder (h = width+10, r=6, center = true,$fn=100);
    translate([20,-22,-9]) cube([25,20,10], center=true, $fn=100);
}
difference(){
    translate([-((length+40)/4),-(width+10)/4,-((height)/2+11)]) cube([(length-30)/2,(width+30)/2,20],center=true);
    translate([0,(width)/2,0]) cube([length+25,width,height+50],center = true);  
    translate([-((length+20)/4),0,-((height)/2+11)]) rotate ([90,0,0]) cylinder (h = 25, r=5, center = true, $fn=100);
}



difference(){
    translate([((length+40)/4),-(width+10)/4,-((height)/2+11)]) cube([(length-30)/2,(width+30)/2,20],center=true);
    translate([0,(width)/2,0]) cube([length+25,width,height+50],center = true);  
    translate([((length+20)/4),0,-((height)/2+11)]) rotate ([90,0,0]) cylinder (h = 25, r=5, center = true, $fn=100);
}

}



