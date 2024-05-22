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
  techReqs = [
"Projector/Screen" ,
"Special presentation specific lighting" ,
"Additional handheld microphones" ,
"On body \"Lav\" microphones",
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
  submitMX() {
    if (!this.mxform.value.date) {return;}
    // @ts-ignore
    var newdate = new Date(this.mxform.value.date);
    var date = dateformat(newdate);

    if (!this.mxform.value.title || !this.mxform.value.description ) {
      return
    }
    // let date = dateformat(this.mxform.value.date)
    this.mxManager.postMx(
      date,
      <string>this.mxform.value.title,
      <string>this.mxform.value.description,
    )
    this.mxform.reset()
    this.route.navigate(['']);

  }

  mxform = new FormGroup({
    date:  new FormControl('', Validators.required),
    title: new FormControl('', Validators.required),
    description: new FormControl('', Validators.required),
  })
  prefform = new FormGroup({
    date:  new FormControl('',{nonNullable: true}),
    title: new FormControl('', {nonNullable: true}),
    description: new FormControl('', {nonNullable: true}),
  })

  next() {
    if (this.formpage == 0 && !this.mxform.value.title || !this.mxform.value.description ) {
      return
    }

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

