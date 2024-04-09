import {Component} from '@angular/core';
import {AuthorizationService} from "../authorization.service";
import {RouterOutlet} from "@angular/router";
import {CookieService} from "ngx-cookie-service";
import {FormControl, FormGroup, ReactiveFormsModule} from "@angular/forms";
import {MorningExService} from "../morning-ex.service";
import {MatFormFieldModule, MatHint} from "@angular/material/form-field";
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatInputModule} from '@angular/material/input';
import {provideNativeDateAdapter} from "@angular/material/core";


@Component({
  selector: 'app-homepage',
  standalone: true,
  imports: [
    RouterOutlet,
    ReactiveFormsModule,
    MatHint,

    MatFormFieldModule, MatInputModule, MatDatepickerModule
  ],
  providers: [provideNativeDateAdapter()],
  templateUrl: './homepage.component.html',
  styleUrl: './homepage.component.css'
})
export class HomepageComponent {
  submitMX() {
    // @ts-ignore
    var date = dateformat(this.mxform.value.date)
    console.log(date)

    if (!this.mxform.value.title || !this.mxform.value.date ) {
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
    date:  new FormControl('',{nonNullable: true}),
    title: new FormControl('', {nonNullable: true}),
    description: new FormControl('', {nonNullable: true}),
  })

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

