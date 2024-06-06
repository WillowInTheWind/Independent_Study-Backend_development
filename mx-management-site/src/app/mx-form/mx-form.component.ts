import {Component} from '@angular/core';
import {AuthorizationService} from "../authorization.service";
import {Router, RouterOutlet} from "@angular/router";
import {CookieService} from "ngx-cookie-service";
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from "@angular/forms";
import {MorningExService} from "../morning-ex.service";
import {MatFormFieldModule, MatHint} from "@angular/material/form-field";
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatInputModule} from '@angular/material/input';
import {MatOption, NativeDateAdapter, provideNativeDateAdapter} from "@angular/material/core";
import {CommonModule, formatDate} from "@angular/common";
import {MatCheckbox} from "@angular/material/checkbox";
import {MatSelect} from "@angular/material/select";
import {MatSelectModule} from '@angular/material/select';
import { DateAdapter } from '@angular/material/core';


@Component({
  selector: 'app-mx-form',
  standalone: true,
  imports: [CommonModule,
    RouterOutlet,
    ReactiveFormsModule,
    MatHint,
    MatFormFieldModule,
    MatInputModule, MatDatepickerModule, MatCheckbox, MatSelectModule
  ],
  providers: [
    provideNativeDateAdapter()
    ],
  templateUrl: './mx-form.component.html',
  styleUrl: './mx-form.component.css'
})
export class MxFormComponent {
  show: string = "";

  counter(i: number) {
    return new Array(i);
  }
  grades = [
    'JK',
    'SK',
    '1st',
    '2nd',
    '3rd',
    '4th',
    '5th',
    '6th',
    '7th',
    '8th',
    '9th',
    '10th',
    '11th',
    '12th'
  ]
  techReqs = {
    Projector: "Projector/Screen",
    PresentationSpecificLighting: "Special presentation specific lighting",
    HandheldMics: "Additional handheld microphones",
    LavMics: "On body \"Lav\" microphones",
  }
  mxdesc = [
    "Extension of an UPPER SCHOOL course or activity\n" ,
    "Extension of a MIDDLE SCHOOL course or activity\n" ,
    "Extension of an INTERMEDIATE SCHOOL course or activity\n" ,
    "Extension of a LOWER SCHOOL course or activity\n" ,
    "Student Musical Performance\n" ,
    "Outside Musical Perfromance\n" ,
    "Outside Speaker\n" ,
    "Student Theatre Performance\n" ,
    "Outside Theatre Performance"
  ]
  mxDetails = [
    "MX goes from 10:55 until 11:25. " +
    "There are NO extended MXs." +
    " Programs should be prepared to last 30 minutes or 25 minutes with 5 minutes for questions.",
    "Please be specific in the presentation description on the form––it will be used as the official description and shared with the entire community" ,
    "MXs should allow time for students to be seated and for announcements after the presentation.",
    "Once scheduled, if this MX needs to be cancelled, it is the faculty sponsor’s responsibility to work with the MX committee to schedule or plan a suitable replacement.            ",
    "Tech will automatically provide one microphone, standard stage lighting, and a podium. Any other tech needs should have listed in the prior section.               ",
    "If special cues are needed, any video, power point, or music, along with a full script of the presentation, should be sent at least one week prior to performance, either through the MX website or email,  to allow adequate technical rehearsal."
  ]
  protected formpage: number = 0;
  submitMX() {
    if (!this.basicdetailsform.value.date) {return;}
    if (!this.basicdetailsform.value.title || !this.basicdetailsform.value.description ) {return}
    var required_tech_json: string = "";
    console.log(this.techform.value.Projector)
    if (this.techform.value.Projector){
      required_tech_json += "Projector/Screen::";
    }
    if ( this.techform.value.PresentationSpecificLighting ){
      required_tech_json += "Special presentation specific lighting::";
    }
    if ( this.techform.value.HandheldMics ){
      required_tech_json += "Additional handheld microphones::";
    }
    if (    this.techform.value.LavMics ){
      required_tech_json += "On body Lav microphones::";
    }
    required_tech_json += this.techform.value.additionalreqs;
    // @ts-ignore
    var newdate = new Date(this.basicdetailsform.value.date);
    var date = dateformat(newdate);
    var title: string = <string>this.basicdetailsform.value.title;
    var description: string = <string>this.basicdetailsform.value.description;
    // @ts-ignore
    var min_grade: number = this.grades.indexOf(<string>this.prefform.value.mingrade);
    // @ts-ignore
    var max_grade: number =  this.grades.indexOf(<string>this.prefform.value.maxgrade);
    var young_student_prep_instructions: string = <string>this.prefform.value.young_student_prep_instructions;
    var is_available_in_day: boolean = <string>this.prefform.value.is_available_in_day=="true";
    var short_description: string = <string>this.shortdescfoorm.value.shortdescription;
    var editors_json: string = "";
    var is_approved: boolean = false;
    this.mxManager.postMx(
      date,
      title,
      description,
      min_grade,
      max_grade,
      young_student_prep_instructions,
      is_available_in_day,
      required_tech_json,
      short_description,
      editors_json,
      is_approved,
    )
          // this.basicdetailsform.reset()
    this.route.navigate(['']);

  }

  basicdetailsform = new FormGroup({
    date:  new FormControl('', Validators.required),
    title: new FormControl('', Validators.required),
    description: new FormControl('', Validators.required),
  })
  prefform = new FormGroup({
    mingrade:  new FormControl('',{nonNullable: true}),
    maxgrade: new FormControl('', {nonNullable: true}),
    young_student_prep_instructions: new FormControl('', {nonNullable: true}),
    is_available_in_day: new FormControl('', {nonNullable: true})
  })
  techform = new FormGroup({
    Projector:  new FormControl('',{nonNullable: true}),
    PresentationSpecificLighting: new FormControl('', {nonNullable: true}),
    HandheldMics: new FormControl('', {nonNullable: true}),
    LavMics: new FormControl('', {nonNullable: true}),
    additionalreqs: new FormControl('', {nonNullable: true})
  })
  shortdescfoorm = new FormGroup({
    shortdescription:  new FormControl('',{nonNullable: true})
  })

  next() {
    if (this.formpage == 0 && !this.basicdetailsform.value.title || !this.basicdetailsform.value.description ) {
      return
    }
    // if (this.formpage == 1 && !this.prefform.value.mingrade || !this.prefform.value.maxgrade ) {
    //   return
    // }
    this.formpage++
  }
  constructor(private route: Router, private dateAdapter: DateAdapter<Date>, private hello: AuthorizationService, private cookies: CookieService, private mxManager: MorningExService) {
    // this.login()
    this.dateAdapter.setLocale('en-US');

  }
  myFilter = (d: Date | null): boolean => {
    // @ts-ignore
    const day = (d || new Date()).getDay();

    return day == 1 || day == 5;
  };
}

function dateformat(date: Date): string {
  //
  if ((date.getMonth() - 10) < 0) {
    var month = "0" + (date.getMonth()+1).toString()
  }
  else{
    var month = (date.getMonth()+1).toString()
  }
  if (date.getDate() - 10 < 0) {
    var day = "0" + date.getDate().toString()
  }
  else{
    var day = date.getDate().toString()
  }
  return (date.getFullYear().toString() + "-" + month + "-" + day);
}

