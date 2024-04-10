import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class MorningExService {
  postMx (
          date:string,
          title: string,
          description: string
  )
  {
    let morning_ex = {
      "date": date,
      "title": title,
      "description": description
    };
    console.log(morning_ex)
    this.http.post("/api/morningexercises/create", morning_ex).subscribe(date => {})
  }
  editMx () {

  }
  deletemx () {

  }
  getmx () {

  }
  getusermxs() {
    console
    return this.http.get<MorningExercise[]>("/api/morningexercises/mine")
  }
  approveMx(title: string) {
    this.http.post("/api/morningexercises/approve", {title}).subscribe(data => {
      console.log(data)
    })
  }
  getallmxs () {
    return this.http.get<MorningExercise[]>("/api/morningexercises")
  }
  constructor(private http: HttpClient) { }
}

export interface MorningExercise{
  id: number,
  mx_index: number,
  date: string,
  owner: {
    sub: string,
    pictrue: string,
    email: string
    name: string
  },
  title: string,
  description: string,
  // editors: Vec<GenericUser>
}

