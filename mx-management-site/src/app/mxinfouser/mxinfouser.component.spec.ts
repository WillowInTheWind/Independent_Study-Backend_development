import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MxinfouserComponent } from './mxinfouser.component';

describe('MxinfouserComponent', () => {
  let component: MxinfouserComponent;
  let fixture: ComponentFixture<MxinfouserComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MxinfouserComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MxinfouserComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
