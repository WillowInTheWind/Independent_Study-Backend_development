import {Component} from '@angular/core';
import {AuthorizationService} from "../authorization.service";
import {RouterOutlet} from "@angular/router";
import {CookieService} from "ngx-cookie-service";
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from "@angular/forms";
import {MorningExService} from "../morning-ex.service";
import {MatFormFieldModule, MatHint} from "@angular/material/form-field";
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatInputModule} from '@angular/material/input';
import {MatOption, provideNativeDateAdapter} from "@angular/material/core";
import {CommonModule} from "@angular/common";
import {MatCheckbox} from "@angular/material/checkbox";
import {MatSelect} from "@angular/material/select";
import {MatSelectModule} from '@angular/material/select';


@Component({
  selector: 'app-mx-form',
  standalone: true,
  imports: [CommonModule,
    RouterOutlet,
    ReactiveFormsModule,
    MatHint,
    MatFormFieldModule, MatInputModule, MatDatepickerModule, MatCheckbox, MatSelectModule
  ],
  providers: [provideNativeDateAdapter()],
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
 '12th',

  ]
  protected formpage: number = 0;
  submitMX() {
    // @ts-ignore
    var date = dateformat(this.mxform.value.date)
    console.log(date)

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
    if (this.formpage == 1 && !this.prefform.value.title || !this.mxform.value.date ) {
      return
    }

    this.formpage++
  }
  constructor(private hello: AuthorizationService, private cookies: CookieService, private mxManager: MorningExService) {
    // this.login()
  }
  myFilter = (d: Date | null): boolean => {
    const day = (d || new Date()).getDay();

    return day == 1 || day == 5;
  };
}

function dateformat(date: Date): string {

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

