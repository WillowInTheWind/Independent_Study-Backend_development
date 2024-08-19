import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MxFormComponent } from './mx-form.component';

describe('HomepageComponent', () => {
  let component: MxFormComponent;
  let fixture: ComponentFixture<MxFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MxFormComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MxFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
