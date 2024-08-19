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
import {filter} from "rxjs";
import {MxinfoComponent} from "../mxinfo/mxinfo.component";
import {MatDialog} from "@angular/material/dialog";
import {ErrorPageComponent} from "../error-page/error-page.component";


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
  async submitMX() {
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
    let post = await this.mxManager.postMx(
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
    if (post) { await this.route.navigate(['mx/success']); return; }
    window.alert("There was a mistake and something went wrong, Try again later")
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
  constructor(private route: Router,     private dialog: MatDialog,
              private dateAdapter: DateAdapter<Date>, private hello: AuthorizationService, private cookies: CookieService, private mxManager: MorningExService) {
    // this.login()
    this.dateAdapter.setLocale('en-US');

  }

  ngOnInit () {
    if (!this.cookies.get("__session") ) {
      const dialogRef = this.dialog.open(ErrorPageComponent, {
        width: '90vw',
        data: "You are not Logged in",
      });
    }
  }


  myFilter = (d: Date | null): boolean => {
    // @ts-ignore

    return (getCurrentDay(d) == 3 || getCurrentDay(d) == 7);
  };
}

function dateformat(date: Date): string {
  //
  if ((date.getMonth() - 9) < 0) {
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


var offdays = [["September 2 2024", "September 2 2024"], ["October 11 2024", "October 11 2024"], [ "October 14 2024", "October 14 2024"], [ "November 27 2024", "November 29 2024"], [ "December 20 2024", "January 3 2025"], [ "January 6 2025", "January 6 2025"], [ "January 17 2025", "January 17 2025"], [ "January 20 2025", "January 21 2025"], [ "February 17 2025", "February 21 2025"], [ "April 7 2025", "April 11 2025"], [ "April 14 2025", "April 14 2025"], [ "May 26 2025", "May 26 2025"]];
function getCurrentDay(today: Date) {
  let year = new Date().getFullYear();

  let start:  Date;
  if (new Date("1/22/25" + year.toString()).getTime() < new Date().getTime()) {
    start = new Date("1/21/25 23:59");
  } else {
    start = new Date("9/3/2024 23:59");
  }
  const offdays_parsed = offdays.map(e => {
      let startDate = new Date(e[0]+", 0:00")
      let endDate = new Date(e[1]+", 23:59")
      if (startDate.getMonth() > 7) {
        startDate.setFullYear(2024)
      } else {
        startDate.setFullYear(2025)
      }
      if (endDate.getMonth() > 7) {
        endDate.setFullYear(2024)
      } else {
        endDate.setFullYear(2025)
      }
      return [startDate, endDate]
    }
  )
  if (today <= new Date("9/4/24") || today >= new Date("6/5/25")|| offdays_parsed.some(e => e[0].getTime() <= today.getTime() && e[1].getTime() >= today.getTime()) || today.getDay() === 0 || today.getDay() === 6) {
      return 0;

  }
  let days = 0
  while (start.getTime() <= today.getTime()) {
    let day = start.getDay()
    if (day === 0 || day === 6 || offdays_parsed.some(e => e[0].getTime() <= start.getTime() && e[1].getTime() >= start.getTime()) ) {
      start.setDate(start.getDate() + 1)
      continue;
    }
    start.setDate(start.getDate() + 1)
    days += 1;
  }
  return days % 8;
}
